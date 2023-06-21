USE de_releases;

CREATE TABLE IF NOT EXISTS config_defaults (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    section_id INT NOT NULL,
    cfg_key TEXT NOT NULL,
    cfg_value TEXT NOT NULL,
    value_type_id INT NOT NULL,

    FOREIGN KEY (section_id) REFERENCES config_sections(id),
    FOREIGN KEY (value_type_id) REFERENCES config_value_types(id)
);