-- Your SQL goes here
CREATE TABLE IF NOT EXISTS martial_arts (
  martialart_name CHARACTER(36) NOT NULL PRIMARY KEY,
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