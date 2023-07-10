USE de_releases;

WITH 
    js_template_id AS (
        SELECT id FROM config_templates WHERE path = 'templates/configs/jobservices.yml'
    ),
    de_env_id AS (
        SELECT id FROM environments WHERE name = 'de'
    )
INSERT INTO environments_services_config_templates
    (environment_service_id, config_template_id, path)
VALUES 
    (
        (
            SELECT id 
            FROM environments_services 
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id 
                FROM services 
                WHERE name = 'analyses'
            )
        ),
        (
            SELECT id 
            FROM config_templates 
            WHERE path = 'templates/configs/analyses.properties'
        ),
        '/etc/iplant/de/analyses.properties'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'app-exposer'
            )
        ),
        (SELECT id FROM js_template_id),
        '/etc/cyverse/de/configs/service.yml'
    );