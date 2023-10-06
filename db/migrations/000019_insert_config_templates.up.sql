USE de_releases;

INSERT INTO config_templates
    (repo_id, path)
VALUES
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/analyses.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/apps.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/bulk-typer.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/ceph-test-vars.txt'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/clockwork.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/dashboard-aggregator.yaml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/data-info.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/data-usage-api.yml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/de-application.yaml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/de-test-vars.yaml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/dewey.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/emailservice.yml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/group-propagator.yml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/grouper-cache.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/grouper-client.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/grouper-hibernate.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/grouper-loader.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/grouper-log4j.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/grouper-morph-string.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/grouper-realm.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/grouper-subject.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/grouper-ws.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/grouper.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/info-typer.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/infosquito.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/infosquito2.yml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/iplant-groups.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/jobservices.yml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/kifshare.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/legacy-de-test-vars.txt'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/metadata.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/monkey.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/notificationagent.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/permissions.yaml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/search.yaml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/sonora.yaml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/templeton.yaml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/terrain.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/vice-default-backend.yml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/vice-image-cache.yml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/webhooks.yml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/secrets/secrets.yml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/exim-sender.yml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/grouper-loader.yml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/grouper-ws.yml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/notification-agent.yml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/tree-urls.yml'
    );
