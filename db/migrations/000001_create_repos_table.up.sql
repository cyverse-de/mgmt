-- Contains records of all Git repositories that are needed for a
-- release/deployment.
CREATE TABLE IF NOT EXISTS repos (
    id           SERIAL         PRIMARY KEY,
    url          TEXT           NOT NULL,
    revision     VARCHAR(255)   NOT NULL,
    name         VARCHAR(255)   NOT NULL UNIQUE
);