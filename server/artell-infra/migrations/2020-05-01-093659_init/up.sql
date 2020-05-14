CREATE TABLE artists (
  id          UUID PRIMARY KEY,
  name        TEXT NOT NULL,
  email       TEXT NOT NULL,
  status_msg  TEXT NOT NULL,
  description TEXT NOT NULL,
  instagram   TEXT NOT NULL,
  twitter     TEXT NOT NULL
);

CREATE TABLE arts (
  id            UUID PRIMARY KEY,
  artist_id     UUID NOT NULL,
  title         TEXT NOT NULL,
  image_name    TEXT NOT NULL,
  portfolio_id  TEXT NOT NULL,

  CONSTRAINT art_artist_fkey FOREIGN KEY (artist_id)
    REFERENCES artists (id) ON UPDATE RESTRICT ON DELETE RESTRICT
);

CREATE TABLE schedules (
  id            SERIAL PRIMARY KEY,
  art_id        UUID NOT NULL,
  activate_at   TIMESTAMPTZ NOT NULL,
  is_scheduled  BOOLEAN NOT NULL DEFAULT TRUE,

  CONSTRAINT schedule_art_fkey FOREIGN KEY (art_id)
    REFERENCES arts (id) ON UPDATE RESTRICT ON DELETE RESTRICT
);
