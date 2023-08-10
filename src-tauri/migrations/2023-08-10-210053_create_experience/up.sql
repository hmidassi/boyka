-- Your SQL goes here
CREATE TABLE IF NOT EXISTS experience (
    id INTEGER NOT NULL PRIMARY KEY,
    club_name  CHARACTER(60),
    starting_date DATETIME NOT NULL,
    end_date DATETIME,
    martial_artist_id INTEGER
);