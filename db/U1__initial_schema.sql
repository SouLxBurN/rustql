CREATE TABLE author (
    id SERIAL,
    name text NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE article (
    id SERIAL,
    title text NOT NULL,
    body text NOT NULL,
    language text NOT NULL,
    author_id integer NOT NULL REFERENCES author,
    PRIMARY KEY(id)
);

CREATE TABLE tag (
    id SERIAL,
    name text NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE article_tag (
    article_id integer NOT NULL REFERENCES article,
    tag_id integer NOT NULL REFERENCES tag
);
