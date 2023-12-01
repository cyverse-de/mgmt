INSERT INTO environments_services 
    (environment_id, service_id)
SELECT 
    (SELECT id FROM environments WHERE name = 'de'), 
    services.id 
FROM services;