USE de_releases;

-- Maps services to config templates and config values.
CREATE TABLE IF NOT EXISTS services_config_templates (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    service_id INT NOT NULL,
    config_template_id INT NOT NULL,

    -- The path to the rendered config file.
    path TEXT NOT NULL, 

    FOREIGN KEY (service_id) REFERENCES services(id),
    FOREIGN KEY (config_template_id) REFERENCES config_templates(id)
);
