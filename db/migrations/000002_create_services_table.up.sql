USE de_releases;

-- Contains records of all services that are deployed in a release.
CREATE TABLE IF NOT EXISTS services (
    id              INT             NOT NULL AUTO_INCREMENT PRIMARY KEY,
    repo_id         INT             NOT NULL,
    name            VARCHAR(255)    NOT NULL,
    skaffold_path   TEXT            NOT NULL,

    FOREIGN KEY (repo_id) REFERENCES repos(id) ON DELETE CASCADE
);