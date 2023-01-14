CREATE TABLE IF NOT EXISTS articles
(
        id            SERIAL PRIMARY KEY,
        -- pub_date      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        wiki_id       INTEGER NOT NULL,
        title         TEXT NOT NULL,
        content       TEXT NOT NULL,
        views         INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS public.users (
    id serial NOT NULL,
    id_session int8 NOT NULL,
    t_email VARCHAR(128) UNIQUE NOT NULL,
    t_password VARCHAR(128) NOT NULL,
    t_ip_address VARCHAR(32) NOT NULL,
    d_visit_first DATE NOT NULL,
    CONSTRAINT users_pkey PRIMARY KEY (id)
);
CREATE UNIQUE INDEX IF NOT EXISTS users_id_idx ON public.users USING btree (id);

CREATE TABLE IF NOT EXISTS categories
(
        id            SERIAL PRIMARY KEY,
        article_id    INTEGER NOT NULL,
        category      TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS completed_pages
(
        id            SERIAL PRIMARY KEY,
        user_id       INTEGER NOT NULL,
        article_id    INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS reports
(
        id            SERIAL PRIMARY KEY,
        article_id    INTEGER NOT NULL,
        report_cat    TEXT NOT NULL,
        date          DATE NOT NULL,
        description   TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS games
(
        id            SERIAL PRIMARY KEY,
        article_id    INTEGER NOT NULL,
        ip_or_email   TEXT NOT NULL,
        is_ip         BOOLEAN NOT NULL,
        is_finished   BOOLEAN NOT NULL,
        words         TEXT NOT NULL
);
