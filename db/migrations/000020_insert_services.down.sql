USE de_releases;

DELETE FROM services
WHERE services.name IN (
    SELECT name
    FROM repos
    WHERE url LIKE 'https://github.com/cyverse-de/%'
);