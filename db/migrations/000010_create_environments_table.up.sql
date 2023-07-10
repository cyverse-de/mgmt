USE de_releases;

CREATE TABLE IF NOT EXISTS environments (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    namespace VARCHAR(255) NOT NULL,
    UNIQUE (name),
    UNIQUE (namespace)
);