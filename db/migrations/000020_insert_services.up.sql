USE de_releases;

INSERT INTO services 
    (name, skaffold_path, repo_id)
VALUES 
    ('analyses', 'skaffold.yaml', (SELECT id FROM repos WHERE name = 'analyses')),
    ('app-exposer', 'skaffold.yaml', (SELECT id FROM repos WHERE name = 'app-exposer')),
    ('apps', 'skaffold.yaml', (SELECT id FROM repos WHERE name = 'apps')),
    ('async-tasks', 'skaffold.yaml', (SELECT id FROM repos WHERE name = 'async-tasks')),
    ('bulk-typer', 'skaffold.yaml', (SELECT id FROM repos WHERE name = 'bulk-typer')),
    ('check-resource-access', 'skaffold.yaml', (select id FROM repos where name = 'check-resource-access')),
    ('clockwork', 'skaffold.yaml', (select id from repos where name = 'clockwork')),
    ('dashboard-aggregator', 'skaffold.yaml', (select id from repos where name = 'dashboard-aggregator')),
    ('data-info', 'skaffold.yaml', (select id from repos where name = 'data-info')),
    ('data-usage-api', 'skaffold.yaml', (select id from repos where name = 'data-usage-api')),
    ('de-mailer', 'skaffold.yaml', (select id from repos where name = 'de-mailer')),
    ('de-webhooks', 'skaffold.yaml', (select id from repos where name = 'de-webhooks')),
    ('dewey', 'skaffold.yaml', (select id from repos where name = 'dewey')),
    ('discoenv-analyses', 'skaffold.yaml', (select id from repos where name = 'discoenv-analyses')),
    ('discoenv-users', 'skaffold.yaml', (select id from repos where name = 'discoenv-users')),
    ('email-requests', 'skaffold.yaml', (select id from repos where name = 'email-requests')),
    ('event-recorder', 'skaffold.yaml', (select id from repos where name = 'event-recorder')),
    ('get-analysis-id', 'skaffold.yaml', (select id from repos where name = 'get-analysis-id')),
    ('group-propagator', 'skaffold.yaml', (select id from repos where name = 'group-propagator')),
    ('grouper-loader', 'skaffold.yaml', (select id from repos where name = 'grouper-dockerfile')),
    ('grouper-ws', 'skaffold.yaml', (select id from repos where name = 'grouper-dockerfile')),
    ('info-typer', 'skaffold.yaml', (select id from repos where name = 'info-typer')),
    ('infosquito2', 'skaffold.yaml', (select id from repos where name = 'infosquito2')),
    ('iplant-groups', 'skaffold.yaml', (select id from repos where name = 'iplant-groups')),
    ('jex-adapter', 'skaffold.yaml', (select id from repos where name = 'jex-adapter')),
    ('job-status-listener', 'skaffold.yaml', (select id from repos where name = 'job-status-listener')),
    ('job-status-recorder', 'skaffold.yaml', (select id from repos where name = 'job-status-recorder')),
    ('job-status-to-apps-adapter', 'skaffold.yaml', (select id from repos where name = 'job-status-to-apps-adapter')),
    ('kifshare', 'skaffold.yaml', (select id from repos where name = 'kifshare')),
    ('metadata', 'skaffold.yaml', (select id from repos where name = 'metadata')),
    ('monkey', 'skaffold.yaml', (select id from repos where name = 'monkey')),
    ('notifications', 'skaffold.yaml', (select id from repos where name = 'notifications')),
    ('permissions', 'skaffold.yaml', (select id from repos where name = 'permissions')),
    ('qms', 'skaffold.yaml', (select id from repos where name = 'qms')),
    ('qms-adapter', 'skaffold.yaml', (select id from repos where name = 'qms-adapter')),
    ('requests', 'skaffold.yaml', (select id from repos where name = 'requests')),
    ('resource-usage-api', 'skaffold.yaml', (select id from repos where name = 'resource-usage-api')),
    ('search', 'skaffold.yaml', (select id from repos where name = 'search')),
    ('sonora', 'skaffold.yaml', (select id from repos where name = 'sonora')),
    ('subscriptions', 'skaffold.yaml', (select id from repos where name = 'subscriptions')),
    ('templeton-incremental', 'skaffold.yaml', (select id from repos where name = 'templeton')),
    ('templeton-periodic', 'skaffold.yaml', (select id from repos where name = 'templeton')),
    ('terrain', 'skaffold.yaml', (select id from repos where name = 'terrain')),
    ('timelord', 'skaffold.yaml', (select id from repos where name = 'timelord')),
    ('user-info', 'skaffold.yaml', (select id from repos where name = 'user-info')),
    ('vice-default-backend', 'skaffold.yaml', (select id from repos where name = 'vice-default-backend')),
    ('vice-status-listener', 'skaffold.yaml', (select id from repos where name = 'vice-status-listener'));
