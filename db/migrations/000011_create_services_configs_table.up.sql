USE de_releases;

-- Maps services to config templates and config values.
CREATE TABLE IF NOT EXISTS services_configs (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    service_id INT NOT NULL,
    config_template_id INT NOT NULL,
    config_value_id INT NOT NULL,
    path TEXT NOT NULL,

    FOREIGN KEY (service_id) REFERENCES services(id),
    FOREIGN KEY (config_template_id) REFERENCES config_templates(id),
    FOREIGN KEY (config_value_id) REFERENCES services_config_values(id)
);
