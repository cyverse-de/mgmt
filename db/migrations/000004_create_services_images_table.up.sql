USE de_releases;

CREATE TABLE IF NOT EXISTS services_images (
    id           INT            NOT NULL AUTO_INCREMENT PRIMARY KEY,
    service_id   INT            NOT NULL,
    image_id     INT            NOT NULL,

    FOREIGN KEY (service_id) REFERENCES services(id),
    FOREIGN KEY (image_id) REFERENCES container_images(id)
);