-- Records the config values that are available to services
-- within the environments. Does not actually associate a config value
-- to a service. That is done in the environments_services_config_values
-- table.
CREATE TABLE IF NOT EXISTS environments_config_values (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    environment_id INT NOT NULL,
    config_value_id INT NOT NULL,

    FOREIGN KEY (environment_id) REFERENCES environments(id) ON DELETE CASCADE,
    FOREIGN KEY (config_value_id) REFERENCES config_values(id) ON DELETE CASCADE,
    UNIQUE (environment_id, config_value_id)
);