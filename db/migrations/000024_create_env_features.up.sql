USE de_releases;

-- Tells whether a feature is enabled in an environment.
CREATE TABLE IF NOT EXISTS environments_features (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    environment_id INT NOT NULL,
    administration boolean NOT NULL DEFAULT false,
    analytics boolean NOT NULL DEFAULT false,
    agave boolean NOT NULL DEFAULT false,
    base_urls boolean NOT NULL DEFAULT false,
    cas boolean NOT NULL DEFAULT false,
    docker boolean NOT NULL DEFAULT false,
    infosquito boolean NOT NULL DEFAULT false,
    intercom boolean NOT NULL DEFAULT false,
    jaeger boolean NOT NULL DEFAULT false,
    jobs boolean NOT NULL DEFAULT false,
    jvmopts boolean NOT NULL DEFAULT false,
    permanent_id boolean NOT NULL DEFAULT false,
    qa boolean NOT NULL DEFAULT false,
    qms boolean NOT NULL DEFAULT false,
    unleash boolean NOT NULL DEFAULT false,

    FOREIGN KEY (environment_id) REFERENCES environments(id) ON DELETE CASCADE,
    UNIQUE (environment_id)
);