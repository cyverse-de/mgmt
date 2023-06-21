USE de_releases;

INSERT INTO services (name, skaffold_path, repo_id)
SELECT repos.name, 'skaffold.yaml', repos.id
FROM repos
WHERE url LIKE 'https://github.com/cyverse-de/%';