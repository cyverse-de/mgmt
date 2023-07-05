USE de_releases;

CREATE TABLE IF NOT EXISTS environments_services_config_values (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    environment_service_id INT NOT NULL,
    environment_config_value_id INT NOT NULL,

    FOREIGN KEY (environment_service_id) REFERENCES environments_services(id),
    FOREIGN KEY (environment_config_value_id) REFERENCES environments_config_values(id),
    UNIQUE (environment_service_id, environment_config_value_id)
);