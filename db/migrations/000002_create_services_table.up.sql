-- Contains records of all services that are deployed in a release.
CREATE TABLE IF NOT EXISTS services (
    id              SERIAL          PRIMARY KEY,
    repo_id         INT             NOT NULL REFERENCES repos(id) ON DELETE CASCADE,
    name            VARCHAR(255)    NOT NULL,
    skaffold_path   VARCHAR(255)    NOT NULL
);
