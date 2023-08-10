-- Your SQL goes here
CREATE TABLE IF NOT EXISTS experience (
    id INTEGER NOT NULL PRIMARY KEY,
    session_date DATETIME NOT NULL,
        session_duration_min INTEGER,
        martial_art_name CHARACTER(36) NOT NULL,
        punches INTEGER,
        kicks INTEGER,
        knees INTEGER,
        elbows INTEGER,
        standup_grappling INTEGER,
        ground_grappling INTEGER,
        ground_n_pound INTEGER,
        trapping INTEGER,
        weapon_manipulation INTEGER,
        weapon_defense INTEGER
);