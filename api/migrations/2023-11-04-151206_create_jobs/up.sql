CREATE TABLE jobs (
  id VARCHAR PRIMARY KEY,
  document_title VARCHAR NOT NULL,
  document_size_in_bytes int NOT NULL,
  status VARCHAR NOT NULL
)