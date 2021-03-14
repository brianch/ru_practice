-- Your SQL goes here
CREATE TABLE nouns (
    id_noun		INTEGER  NOT NULL PRIMARY KEY,
    gender		TEXT	 NOT NULL,
    animate		INTEGER  NOT NULL,
    sing_nominative	TEXT,
    sing_genitive	TEXT,
    sing_dative		TEXT,
    sing_accusative	TEXT,
    sing_instrumental	TEXT,
    sing_prepositional	TEXT,
    sing_locative	TEXT,
    plur_nominative     TEXT,
    plur_genitive       TEXT,
    plur_dative         TEXT,
    plur_accusative     TEXT,
    plur_instrumental   TEXT,
    plur_prepositional  TEXT,
    plur_locative       TEXT
);
