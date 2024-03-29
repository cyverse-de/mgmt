-- Records template paths and the repo they belong to.
CREATE TABLE IF NOT EXISTS config_templates (
    id           SERIAL         PRIMARY KEY,
    repo_id      INT            NOT NULL,
    path         TEXT           NOT NULL,

    FOREIGN KEY (repo_id) REFERENCES repos(id) ON DELETE CASCADE
);