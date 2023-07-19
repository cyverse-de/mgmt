USE de_releases;

-- Maps services to one or more container images.
CREATE TABLE IF NOT EXISTS services_images (
    id           INT            NOT NULL AUTO_INCREMENT PRIMARY KEY,
    service_id   INT            NOT NULL,
    image_id     INT            NOT NULL,

    FOREIGN KEY (service_id) REFERENCES services(id) ON DELETE CASCADE,
    FOREIGN KEY (image_id) REFERENCES container_images(id) ON DELETE CASCADE
);