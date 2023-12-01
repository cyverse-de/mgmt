CREATE TABLE IF NOT EXISTS config_defaults (
    id SERIAL PRIMARY KEY,
    section_id INT NOT NULL,
    cfg_key VARCHAR(512) NOT NULL,
    cfg_value TEXT NOT NULL,
    value_type_id INT NOT NULL,

    FOREIGN KEY (section_id) REFERENCES config_sections(id) ON DELETE CASCADE,
    FOREIGN KEY (value_type_id) REFERENCES config_value_types(id) ON DELETE CASCADE
);