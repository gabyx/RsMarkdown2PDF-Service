CREATE TYPE level_t AS ENUM ('info', 'warning', 'error');

CREATE TABLE job_logs (
    -- The unique id of the message.
    id uuid PRIMARY KEY,
    --
    -- The job id this log belongs to.
    job_id uuid NOT NULL,
    CONSTRAINT fk_job FOREIGN KEY (job_id) REFERENCES jobs (
        id
    ) ON DELETE CASCADE,
    --
    -- The timestamp of the log message.
    timestamp timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
    --
    -- The message.
    message text NOT NULL,
    --
    -- The severity of the message.
    level level_t NOT NULL
);
