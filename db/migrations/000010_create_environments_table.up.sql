CREATE TABLE IF NOT EXISTS environments (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    namespace VARCHAR(255) NOT NULL,
    CONSTRAINT unique_name UNIQUE (name),
    CONSTRAINT unique_namespace UNIQUE (namespace)
);