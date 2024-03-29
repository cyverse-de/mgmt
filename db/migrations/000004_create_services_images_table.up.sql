-- Maps services to one or more container images.
CREATE TABLE IF NOT EXISTS services_images (
    id           SERIAL PRIMARY KEY,
    service_id   INT NOT NULL,
    image_id     INT NOT NULL,

    FOREIGN KEY (service_id) REFERENCES services(id) ON DELETE CASCADE,
    FOREIGN KEY (image_id) REFERENCES container_images(id) ON DELETE CASCADE
);