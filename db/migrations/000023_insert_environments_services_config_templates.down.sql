USE de_releases;

DELETE FROM environments_services_config_templates
WHERE environment_service_id IN (
    SELECT id 
    FROM environments_services 
    WHERE environment_id = (
        SELECT id 
        FROM environments 
        WHERE name = 'de'
    )
);