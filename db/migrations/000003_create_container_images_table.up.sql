USE de_releases;

-- Contains records of all container images needed for a release/deployment.
CREATE TABLE IF NOT EXISTS container_images (
    id           INT  NOT NULL AUTO_INCREMENT PRIMARY KEY,
    repo_id      INT  NOT NULL,
    dockerfile   TEXT NOT NULL,
    name         TEXT NOT NULL,
    tag          TEXT NOT NULL,
    digest       TEXT NOT NULL,

    FOREIGN KEY (repo_id) REFERENCES repos(id)
);