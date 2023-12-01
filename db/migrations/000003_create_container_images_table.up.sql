-- Contains records of all container images needed for a release/deployment.
CREATE TABLE IF NOT EXISTS container_images (
    id           INT  NOT NULL AUTO_INCREMENT PRIMARY KEY,
    repo_id      INT  NOT NULL,
    dockerfile   TEXT NOT NULL,
    name         VARCHAR(512) NOT NULL,
    tag          VARCHAR(256) NOT NULL,
    digest       TEXT NOT NULL,

    FOREIGN KEY (repo_id) REFERENCES repos(id) ON DELETE CASCADE,
    UNIQUE (name, tag)
);