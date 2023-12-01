-- Records the config values that are available to services
-- within the environments. Does not actually associate a config value
-- to a service. That is done in the environments_services_config_values
-- table.
CREATE TABLE IF NOT EXISTS environments_config_values (
    id SERIAL PRIMARY KEY,
    environment_id INTEGER NOT NULL,
    config_value_id INTEGER NOT NULL,

    FOREIGN KEY (environment_id) REFERENCES environments(id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (config_value_id) REFERENCES config_values(id) ON DELETE CASCADE ON UPDATE CASCADE
);
