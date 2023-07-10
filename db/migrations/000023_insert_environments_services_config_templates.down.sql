USE de_releases;

WITH 
    env_service_ids AS (
        SELECT id 
        FROM environments_services 
        WHERE environment_id = (
            SELECT id 
            FROM environments 
            WHERE name = 'de'
        )
    )
DELETE FROM environments_services_config_templates
WHERE environment_service_id IN env_service_ids;