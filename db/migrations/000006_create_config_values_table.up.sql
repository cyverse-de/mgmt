USE de_releases;

CREATE TABLE IF NOT EXISTS config_values (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    repo_id INT NOT NULL,
    defaults_file_path TEXT NOT NULL,
    env_file_path TEXT NOT NULL,

    FOREIGN KEY (repo_id) REFERENCES repos(id)
);