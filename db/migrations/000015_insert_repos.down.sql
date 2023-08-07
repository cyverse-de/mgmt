USE de_releases;

DELETE FROM repos 
WHERE url LIKE 'https://github.com/cyverse-de/%';