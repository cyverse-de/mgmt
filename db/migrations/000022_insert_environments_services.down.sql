USE de_releases;

DELETE FROM environments_services 
WHERE environment_id = (
    SELECT id 
    FROM environments 
    WHERE name = 'de'
);