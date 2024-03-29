-- Maps services to config templates in an environment.
CREATE TABLE IF NOT EXISTS environments_services_config_templates (
    id SERIAL PRIMARY KEY,
    environment_service_id INT NOT NULL,
    config_template_id INT NOT NULL,

    -- The path to the rendered config file. Should be set, but is not yet used during deployments.
    path TEXT NOT NULL,

    FOREIGN KEY (environment_service_id) REFERENCES environments_services(id) ON DELETE CASCADE,
    FOREIGN KEY (config_template_id) REFERENCES config_templates(id) ON DELETE CASCADE
);
