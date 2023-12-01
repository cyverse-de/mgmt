-- Maps services to one or more environments.
-- This is a many-to-many relationship.
-- A service can be deployed to many environments.
-- An environment can have many services deployed to it.
-- A service can be used to look up the config values for a given environment.
CREATE TABLE IF NOT EXISTS environments_services (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    environment_id INT NOT NULL,
    service_id INT NOT NULL,

    FOREIGN KEY (environment_id) REFERENCES environments(id) ON DELETE CASCADE,
    FOREIGN KEY (service_id) REFERENCES services(id) ON DELETE CASCADE,
    UNIQUE (environment_id, service_id)
);