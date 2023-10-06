USE de_releases;

WITH 
    js_template_id AS (
        SELECT id FROM config_templates WHERE path = 'templates/jobservices.yml'
    ),
    de_env_id AS (
        SELECT id FROM environments WHERE name = 'de'
    )
INSERT INTO environments_services_config_templates
    (environment_service_id, config_template_id, path)
VALUES 
    (
        (
            SELECT id 
            FROM environments_services 
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id 
                FROM services 
                WHERE name = 'analyses'
            )
        ),
        (
            SELECT id 
            FROM config_templates 
            WHERE path = 'templates/analyses.properties'
        ),
        '/etc/iplant/de/analyses.properties'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'app-exposer'
            )
        ),
        (SELECT id FROM js_template_id),
        '/etc/cyverse/de/configs/service.yml'
    ), 
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'apps'
            )
        ),
        (
            SELECT id 
            FROM config_templates 
            WHERE path = 'templates/apps.properties'
        ),
        '/etc/iplant/de/apps.properties'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'async-tasks'
            )
        ),
        (SELECT id FROM js_template_id),
        '/etc/iplant/de/jobservices.yml'

    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'bulk-typer'
            )
        ),
        (SELECT id FROM config_templates where path = 'templates/bulk-typer.properties'),
        '/etc/iplant/de/bulk-typer.properties'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'clockwork'
            )
        ),
        (SELECT id FROM config_templates where path = 'templates/clockwork.properties'),
        '/etc/iplant/de/clockwork.properties'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'dashboard-aggregator'
            )
        ),
        (SELECT id FROM config_templates where path = 'templates/dashboard-aggregator.yaml'),
        '/etc/cyverse/de/configs/service.yml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'data-info'
            )
        ),
        (SELECT id FROM config_templates where path = 'templates/data-info.properties'),
        '/etc/iplant/de/data-info.properties'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'data-usage-api'
            )
        ),
        (SELECT id FROM config_templates where path = 'templates/data-usage-api.yml'),
        '/etc/iplant/de/data-usage-api.yml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name =  'de-mailer'
            )
        ),
        (SELECT id FROM config_templates where path = 'templates/emailservice.yml'),
        '/etc/iplant/de/emailservice.yml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'de-webhooks'
            )
        ),
        (SELECT id FROM config_templates where path = 'templates/webhooks.yml'),
        '/etc/iplant/de/webhooks.yml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'dewey'
            )
        ),
        (SELECT id FROM config_templates where path = 'templates/dewey.properties'),
        '/etc/iplant/de/dewey.properties'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'discoenv-analyses'
            )
        ),
        (SELECT id FROM js_template_id),
        '/etc/iplant/de/jobservices.yml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'discoenv-users'
            )
        ),
        (SELECT id FROM js_template_id),
        '/etc/cyverse/de/configs/service.yml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'email-requests'
            )
        ),
        (SELECT id FROM js_template_id),
        '/etc/iplant/de/jobservices.yml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'event-recorder'
            )
        ),
        (SELECT id FROM js_template_id),
        '/etc/iplant/de/jobservices.yml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'group-propagator'
            )
        ),
        (SELECT id FROM config_templates WHERE path = 'templates/group-propagator.yml'),
        '/etc/iplant/de/group-propagator.yml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'infosquito2'
            )
        ),
        (SELECT id FROM config_templates where path = 'templates/infosquito2.yml'),
        '/etc/iplant/de/infosquito2.yml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'info-typer'
            )
        ),
        (SELECT id FROM config_templates where path = 'templates/info-typer.properties'),
        '/etc/iplant/de/info-typer.properties'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'iplant-groups'
            )
        ),
        (SELECT id FROM config_templates WHERE path = 'templates/iplant-groups.properties'),
        '/etc/iplant/de/iplant-groups.properties'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'jex-adapter'
            )
        ),
        (SELECT id FROM js_template_id),
        '/etc/iplant/de/jobservices.yml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'job-status-listener'
            )
        ),
        (SELECT id FROM js_template_id),
        '/etc/iplant/de/jobservices.yml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'job-status-recorder'
            )
        ),
        (SELECT id FROM js_template_id),
        '/etc/iplant/de/jobservices.yml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'job-status-to-apps-adapter'
            )
        ),
        (SELECT id FROM js_template_id),
        '/etc/iplant/de/jobservices.yml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'kifshare'
            )
        ),
        (SELECT id FROM config_templates WHERE path = 'templates/kifshare.properties'),
        '/etc/iplant/de/kifshare.properties'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'metadata'
            )
        ),
        (SELECT id FROM config_templates WHERE path = 'templates/metadata.properties'),
        '/etc/iplant/de/metadata.properties'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'monkey'
            )
        ),
        (SELECT id FROM config_templates WHERE path = 'templates/monkey.properties'),
        '/etc/iplant/de/monkey.properties'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'notifications'
            )
        ),
        (SELECT id FROM js_template_id),
        '/etc/iplant/de/jobservices.yml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'permissions'
            )
        ),
        (SELECT id FROM config_templates WHERE path = 'templates/permissions.yaml'),
        '/etc/iplant/de/permissions.yaml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'qms'
            )
        ),
        (SELECT id FROM js_template_id),
        '/etc/cyverse/de/configs/service.yml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'qms-adapter'
            )
        ),
        (SELECT id FROM js_template_id),
        '/etc/iplant/de/jobservices.yml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'requests'
            )
        ),
        (SELECT id FROM js_template_id),
        '/etc/iplant/de/jobservices.yml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'resource-usage-api'
            )
        ),
        (SELECT id FROM js_template_id),
        '/etc/cyverse/de/configs/service.yml'
        ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'search'
            )
        ),
        (SELECT id FROM config_templates WHERE path = 'templates/search.yaml'),
        '/etc/iplant/de/search.yaml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'sonora'
            )
        ),
        (SELECT id FROM config_templates WHERE path = 'templates/sonora.yaml'),
        '/etc/iplant/de/local.yaml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'subscriptions'
            )
        ),
        (SELECT id FROM js_template_id),
        '/etc/cyverse/de/configs/service.yml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'templeton-incremental'
            )
        ),
        (SELECT id FROM config_templates WHERE path = 'templates/templeton.yaml'),
        '/etc/iplant/de/templeton-incremental.yaml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'templeton-periodic'
            )
        ),
        (SELECT id FROM config_templates WHERE path = 'templates/templeton.yaml'),
        '/etc/iplant/de/templeton-periodic.yaml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'terrain'
            )
        ),
        (SELECT id FROM config_templates WHERE path = 'templates/terrain.properties'),
        '/etc/iplant/de/terrain.properties'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'timelord'
            )
        ),
        (SELECT id FROM js_template_id),
        '/etc/iplant/de/jobservices.yml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'user-info'
            )
        ),
        (SELECT id FROM js_template_id),
        '/etc/iplant/de/jobservices.yml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'vice-default-backend'
            )
        ),
        (SELECT id FROM js_template_id),
        '/etc/iplant/de/jobservices.yml'
    ),
    (
        (
            SELECT id
            FROM environments_services
            WHERE environment_id = (SELECT id FROM de_env_id)
            AND service_id = (
                SELECT id
                FROM services
                WHERE name = 'vice-status-listener'
            )
        ),
        (SELECT id FROM js_template_id),
        '/etc/iplant/de/jobservices.yml'
    );