USE de_releases;

CREATE TABLE IF NOT EXISTS repos (
    id           INT            NOT NULL AUTO_INCREMENT PRIMARY KEY,
    url          TEXT           NOT NULL,
    revision     VARCHAR(255)   NOT NULL,
    name         VARCHAR(255)   NOT NULL
);