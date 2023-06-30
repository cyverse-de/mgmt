USE de_releases;

-- Records the config values for a service in an environment.
CREATE TABLE IF NOT EXISTS environments_services_config_values (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    environment_service_id INT NOT NULL,
    config_value_id INT NOT NULL,

    FOREIGN KEY (environment_service_id) REFERENCES environments_services(id),
    FOREIGN KEY (config_value_id) REFERENCES config_values(id),
    UNIQUE (environment_service_id, config_value_id)
);