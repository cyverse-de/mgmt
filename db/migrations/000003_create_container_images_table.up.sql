USE de_releases;

CREATE TABLE IF NOT EXISTS container_images (
    id           INT            NOT NULL AUTO_INCREMENT PRIMARY KEY,
    repo_id      INT            NOT NULL,
    dockerfile   TEXT           NOT NULL,
    registry     VARCHAR(255)   NOT NULL,
    name         VARCHAR(255)   NOT NULL,
    tag          VARCHAR(255)   NOT NULL,

    FOREIGN KEY (repo_id) REFERENCES repos(id)
);