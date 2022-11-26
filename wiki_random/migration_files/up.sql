CREATE TABLE IF NOT EXISTS articles
(
        id            SERIAL PRIMARY KEY,
        -- pub_date      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        wiki_id       INTEGER NOT NULL,
        title         TEXT NOT NULL,
        content       TEXT NOT NULL
);

/* INSERT INTO articles */
/*             (id, */
/*              wiki_id, */
/*              title, */
/*              content) */
/* VALUES      (45288, */
/*              45288, */
/*              'Echinoidea', */
/*              'Les Oursins sont un groupe d''animaux marins, formant la classe */
/* des Echinoidea au sein de l''embranchement des échinodermes. Ils sont aussi */
/* appelés par les scientifiques Échinoïdes ou Échinides.  Ce sont des invertébrés */
/* de forme arrondie au corps recouvert de piquants, ce qui leur vaut d''être */
/* parfois désignés, par analogie, par l''expression populaire de hérissons de mer */
/* et plus rarement par l''expression vieillie de châtaignes de mer.  Comme leurs */
/* proches parents les concombres de mer et les étoiles de mer, ces organismes */
/* benthiques à l''état adulte ont une larve planctonique. ' */
/* ); */



