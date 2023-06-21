USE de_releases;

CREATE TABLE IF NOT EXISTS services_config_values (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    service_id INT NOT NULL,
    config_value_id INT NOT NULL,

    UNIQUE (service_id, config_value_id),
    FOREIGN KEY (service_id) REFERENCES services(id),
    FOREIGN KEY (config_value_id) REFERENCES config_values(id),
    PRIMARY KEY (id)
);