-- Tells whether a feature is enabled in an environment.
CREATE TABLE IF NOT EXISTS environments_features (
    id SERIAL PRIMARY KEY,
    environment_id INT NOT NULL,
    administration BOOLEAN NOT NULL DEFAULT false,
    analytics BOOLEAN NOT NULL DEFAULT false,
    agave BOOLEAN NOT NULL DEFAULT false,
    base_urls BOOLEAN NOT NULL DEFAULT false,
    cas BOOLEAN NOT NULL DEFAULT false,
    docker BOOLEAN NOT NULL DEFAULT false,
    infosquito BOOLEAN NOT NULL DEFAULT false,
    intercom BOOLEAN NOT NULL DEFAULT false,
    jaeger BOOLEAN NOT NULL DEFAULT false,
    jobs BOOLEAN NOT NULL DEFAULT false,
    jvmopts BOOLEAN NOT NULL DEFAULT false,
    permanent_id BOOLEAN NOT NULL DEFAULT false,
    qa BOOLEAN NOT NULL DEFAULT false,
    qms BOOLEAN NOT NULL DEFAULT false,
    unleash BOOLEAN NOT NULL DEFAULT false,

    FOREIGN KEY (environment_id) REFERENCES environments(id),
    UNIQUE (environment_id)
);