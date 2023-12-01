-- Records of config values used to render a template.
CREATE TABLE IF NOT EXISTS config_values (
    id SERIAL PRIMARY KEY,
    section_id INT NOT NULL,
    cfg_key TEXT NOT NULL,
    cfg_value TEXT NOT NULL,
    value_type_id INT NOT NULL,
    default_id INT NOT NULL,

    FOREIGN KEY (section_id) REFERENCES config_sections(id) ON DELETE CASCADE,
    FOREIGN KEY (value_type_id) REFERENCES config_value_types(id) ON DELETE CASCADE,
    FOREIGN KEY (default_id) REFERENCES config_defaults(id) ON DELETE CASCADE
);