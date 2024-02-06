use common::{
    log::{error, info, Logger},
    queue::{DefaultConsumer, StatusQueue},
};
use diesel::{pg::Pg, Connection};
use rocket::{
    fairing::{Fairing, Info, Kind},
    tokio::{
        self, select,
        sync::{
            oneshot::{Receiver, Sender},
            Mutex,
        },
    },
    Shutdown,
};
use std::{sync::Arc, time::Duration};

use crate::database;

/// Rocket fairing to wait for consumer to be done.
pub struct WaitForConsumerDone {
    pub log: Arc<Logger>,

    // Why this strange type:
    // This option wrapped in a mutex seems odd here, but
    // the `Receiver.await` wants to move the receiver because its not Copy.
    // ans because we cannot move out from a `&self` in `on_shutdown` we need
    // interior mutability and because the await can cause things to move to a different thread
    // we need locks. The mutex here gives as internal thread-dafe mutability and the option
    // enables us to move out the value it contains with `.take`.
    // https://users.rust-lang.org/t/how-to-have-joinhandle-inside-rocket-fairing-for-shutdown/105748/7
    pub rx_consumer_done: Mutex<Option<Receiver<()>>>,
}

/// Helper data for the status consumer to react on a shutdown and to acknowledge this
/// by `tx_shutdown`.
pub struct WaitForShutdown {
    pub shutdown: Shutdown,
    pub tx_consumer_done: Sender<()>,
}

#[rocket::async_trait]
impl Fairing for WaitForConsumerDone {
    fn info(&self) -> Info {
        Info {
            name: "Status Consumer Shutdown Fairing",
            kind: Kind::Shutdown,
        }
    }

    async fn on_shutdown(&self, _rocket: &rocket::Rocket<rocket::Orbit>) {
        info!(self.log, "Await status consumer shutdown.");

        let mut m = self.rx_consumer_done.lock().await;
        if let Err(e) = m.take().unwrap().await {
            error!(
                self.log,
                "Could not receive consumer termination message:\n{}", e
            );
        } else {
            info!(self.log, "Consumer successfully terminated.");
        }
    }
}

/// Spawn a status queue consumer thread.
pub async fn spawn_status_consumer(
    l: Arc<Logger>,
    status_queue: StatusQueue,
    database_url: &str,
    wait_for_shutdown: WaitForShutdown,
) {
    let log = l.clone();
    let db_conn = database::connect(&l, &database_url);

    info!(log, "Installing consumer on the status queue.");
    status_queue
        .subscribe(|args| DefaultConsumer::new(log.clone(), args.no_ack))
        .await
        .expect("Could not install consumer.");

    tokio::task::spawn(async move {
        loop {
            select! {
                _ = wait_for_shutdown.shutdown.clone() => {
                    info!(log, "Rocket shutdown received.");
                    break
                },
            }
        }

        info!(log, "Unsubscribe consumer.");
        status_queue.unsubscribe().await;

        // Notify that we finished.
        if let Err(_) = wait_for_shutdown.tx_consumer_done.send(()) {
            error!(
                log,
                "Could not send consumer shutdown message, the receiver dropped."
            )
        }
    });
}
