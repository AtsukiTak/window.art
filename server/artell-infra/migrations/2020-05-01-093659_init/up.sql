CREATE TABLE artists (
  id          UUID PRIMARY KEY,
  name        TEXT NOT NULL,
  email       TEXT NOT NULL,
  status_msg  TEXT NOT NULL,
  description TEXT NOT NULL,
  instagram   TEXT NOT NULL,
  twitter     TEXT NOT NULL
);
