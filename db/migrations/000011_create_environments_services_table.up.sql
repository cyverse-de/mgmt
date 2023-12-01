-- Maps services to one or more environments.
-- This is a many-to-many relationship.
-- A service can be deployed to many environments.
-- An environment can have many services deployed to it.
-- A service can be used to look up the config values for a given environment.
CREATE TABLE IF NOT EXISTS environments_services (
    id SERIAL PRIMARY KEY,
    environment_id INT NOT NULL,
    service_id INT NOT NULL,

    FOREIGN KEY (environment_id) REFERENCES environments(id),
    FOREIGN KEY (service_id) REFERENCES services(id),
    UNIQUE (environment_id, service_id)
);
