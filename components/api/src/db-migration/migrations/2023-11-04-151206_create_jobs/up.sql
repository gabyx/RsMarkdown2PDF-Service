CREATE TYPE status_t AS ENUM ('pending', 'queued', 'running', 'done');
CREATE TYPE result_t AS ENUM ('none', 'success', 'failure');

CREATE TABLE jobs (
    -- The unique id of the job.
    id uuid PRIMARY KEY,
    -- The status of this job.
    status status_t NOT NULL,
    --
    -- The timestamp the job was submitted and done.
    submit_time timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
    end_time timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
    --
    -- The blob which this job converts.
    blob_digest varchar NOT NULL,
    --
    -- The name of the job.
    name varchar NOT NULL,
    --
    -- The converter result and status.
    converter_result result_t NOT NULL,
    converter_log text NOT NULL
);
