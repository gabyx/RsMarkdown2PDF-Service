CREATE TYPE status_t AS ENUM('idle', 'queued', 'running', 'done');
CREATE TYPE result_t AS ENUM('success', 'failure');

CREATE TABLE jobs (
  id varchar PRIMARY KEY,
  --
  -- The name of the job.
  name varchar NOT NULL,
  --
  -- The status of this job.
  status status_t NOT NULL,
  --
  -- The blob which this job converts.
  blob_digest varchar NOT NULL,
  --
  -- The converter result and status.
  converter_result result_t,
  converter_log text
);
