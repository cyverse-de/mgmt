USE de_releases;

-- Maps services to config templates in an environment.
CREATE TABLE IF NOT EXISTS environments_services_config_templates (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    environment_service_id INT NOT NULL,
    config_template_id INT NOT NULL,

    --- The path to the rendered config file.
    path TEXT NOT NULL,

    FOREIGN KEY (environment_service_id) REFERENCES environments_services(id) ON DELETE CASCADE,
    FOREIGN KEY (config_template_id) REFERENCES config_templates(id) ON DELETE CASCADE
);
