-- https://softwaremill.com/implementing-event-sourcing-using-a-relational-database/

CREATE TABLE events (
    id INTEGER PRIMARY KEY,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP NOT NULL,
    stream_id INTEGER NOT NULL,
    version INTEGER NOT NULL,
    data TEXT NOT NULL,
    UNIQUE (stream_id, version)
);
