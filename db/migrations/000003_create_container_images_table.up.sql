-- Contains records of all container images needed for a release/deployment.
CREATE TABLE IF NOT EXISTS container_images (
    id           SERIAL PRIMARY KEY,
    repo_id      INT  NOT NULL,
    dockerfile   TEXT NOT NULL,
    name         VARCHAR(512) NOT NULL,
    tag          VARCHAR(256) NOT NULL,
    digest       TEXT NOT NULL,

    FOREIGN KEY (repo_id) REFERENCES repos(id) ON DELETE CASCADE,
    CONSTRAINT unique_name_tag UNIQUE (name, tag)
);