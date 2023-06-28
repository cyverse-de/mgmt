USE de_releases;

INSERT INTO config_templates
    (repo_id, path)
VALUES
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/analyses.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/apps.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/bulk-typer.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/ceph-test-vars.txt'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/clockwork.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/dashboard-aggregator.yaml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/data-info.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/data-ussage-api.yml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/de-application.yaml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/de-test-vars.yaml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/dewey.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/emailservice.yml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/group-propagator.yml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/grouper-cache.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/grouper-client.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/grouper-hibernate.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/grouper-loader.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/grouper-log4j.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/grouper-morph-string.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/grouper-realm.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/grouper-subject.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/grouper-ws.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/grouper.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/info-typer.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/infosquito.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/infosquito2.yml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/iplant-groups.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/jobservices.yml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/kifshare.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/legacy-de-test-vars.txt'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/metadata.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/monkey.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/notificationagent.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/permissions.yaml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/search.yaml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/sonora.yaml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/templeton.yaml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/terrain.properties'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/vice-default-backend.yml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/vice-image-cache.yml'
    ),
    (
        (SELECT id FROM repos WHERE name = 'de-releases'),
        'templates/configs/webhooks.yml'
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
