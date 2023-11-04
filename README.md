# Markdown to PDF as a service.

## How to run this?

The easiest way to run this is using `tilt` and on a working k8s cluster, such as `k3s`.

### Starting everything

With `tilt` installed and a k8s cluster running:

    cd k8s
    ./deploy.sh


## Development loop (local) for faster feedback

All rust projects can be run locally using `cargo run`, there should a `.env` file that configures the tools to
connect to the services running in the k8s cluster.

## Development loop (k8s) using `tilt`

`tilt` will run all the services and build all the docker containers to the `ttl.sh` registry (ephemeral images).
Then it will watch for changes to any files and k8s manifests and redeploy the services, configmaps as far as possible.