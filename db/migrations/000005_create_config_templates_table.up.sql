USE de_releases;

CREATE TABLE IF NOT EXISTS config_templates (
    id           INT            NOT NULL AUTO_INCREMENT PRIMARY KEY,
    repo_id      INT            NOT NULL,
    path         TEXT           NOT NULL,

    FOREIGN KEY (repo_id) REFERENCES repos(id)
);