USE de_releases;

INSERT INTO config_defaults 
    (section_id, cfg_key, cfg_value, value_type_id) 
VALUES
    (
        (SELECT id FROM config_sections WHERE name = 'Agave'), 
        'Key', 
        '', 
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Agave'),
        'Secret',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Agave'),
        'StorageSystem',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Agave'),
        'CallbackBaseURI',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Agave'), 
        'ReadTimeout', 
        '30000', 
        (SELECT id FROM config_value_types WHERE name = 'int')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Agave'), 
        'Enabled', 
        'false', 
        (SELECT id FROM config_value_types WHERE name = 'bool')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Agave'), 
        'JobsEnabled', 
        'false', 
        (SELECT id FROM config_value_types WHERE name = 'bool')
    );

INSERT INTO config_defaults 
    (section_id, cfg_key, cfg_value, value_type_id)
VALUES
    (
        (SELECT id FROM config_sections WHERE name = 'BaseURLs'), 
        'Analyses', 
        'http://analyses', 
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'BaseURLs'),
        'Apps',
        'http://apps',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'BaseURLs'),
        'AppExposer',
        'http://app-exposer',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'BaseURLs'),
        'AsyncTasks',
        'http://async-tasks',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'BaseURLs'),
        'DashboardAggregator',
        'http://dashboard-aggregator',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'BaseURLs'),
        'DataInfo',
        'http://data-info',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'BaseURLs'),
        'GrouperWebServices',
        'http://grouper-ws/grouper-ws',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'BaseURLs'),
        'IplantEmail',
        'http://de-mailer',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'BaseURLs'),
        'IplantGroups',
        'http://iplant-groups',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'BaseURLs'),
        'JexAdapter',
        'http://jex-adapter',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'BaseURLs'),
        'JobStatusListener',
        'http://job-status-listener',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'BaseURLs'),
        'Metadata',
        'http://metadata',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'BaseURLs'),
        'Notifications',
        'http://notifications',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'BaseURLs'),
        'Permissions',
        'http://permissions',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'BaseURLs'),
        'QMS',
        'http://qms',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'BaseURLs'),
        'Requests',
        'http://requests',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'BaseURLs'),
        'Search',
        'http://search',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'BaseURLs'),
        'Terrain',
        'http://terrain',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'BaseURLs'),
        'UserInfo',
        'http://user-info',
        (SELECT id FROM config_value_types WHERE name = 'string')
    );

INSERT INTO config_defaults
    (section_id, cfg_key, cfg_value, value_type_id)
VALUES
    (
        (SELECT id FROM config_sections WHERE name = 'DashboardAggregator'),
        'PublicGroup',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'DashboardAggregator'),
        'Website.URL',
        'https://cyverse.org',
        (SELECT id FROM config_value_types WHERE name = 'string')
    );

INSERT INTO config_defaults
    (section_id, cfg_key, cfg_value, value_type_id)
VALUES
    (
        (SELECT id FROM config_sections WHERE name = 'DE'),
        'AMQP.User',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'DE'),
        'AMQP.Password',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'DE'),
        'AMQP.Host',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'DE'),
        'AMQP.Port',
        '5672',
        (SELECT id FROM config_value_types WHERE name = 'int')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'DE'),
        'AMQP.Vhost',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'DE'),
        'BaseURI',
        'https://de.cyverse.org',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'DE'),
        'Subscriptions.CheckoutURL',
        'https://cyverse-subscription.phoenixbioinformatics.org',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'DE'),
        'Subscriptions.Enforce',
        'false',
        (SELECT id FROM config_value_types WHERE name = 'bool')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'DE'),
        'DefaultOutputFolder',
        'analyses',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'DE'),
        'Coge.BaseURI',
        'https://genomevolution.org/coge/api/v1',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'DE'),
        'Tools.Admin.MaxCPULimit',
        '24',
        (SELECT id FROM config_value_types WHERE name = 'int')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'DE'),
        'Tools.Admin.MaxMemoryLimit',
        '75161927680',
        (SELECT id FROM config_value_types WHERE name = 'bigint')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'DE'),
        'Tools.Admin.MaxDiskLimit',
        '1099511627776',
        (SELECT id FROM config_value_types WHERE name = 'bigint')
    );

INSERT INTO config_defaults
    (section_id, cfg_key, cfg_value, value_type_id)
VALUES
    (
        (SELECT id FROM config_sections WHERE name = 'Docker'),
        'Tag',
        'latest',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Docker'),
        'TrustedRegistries',
        'harbor.cyverse.org,docker.cyverse.org',
        (SELECT id FROM config_value_types WHERE name = 'csv')
    );

INSERT INTO config_defaults
    (section_id, cfg_key, cfg_value, value_type_id)
VALUES
    (
        (SELECT id FROM config_sections WHERE name = 'Elasticsearch'),
        'BaseURI',
        'http://elasticsearch:9200',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Elasticsearch'),
        'Username',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Elasticsearch'),
        'Password',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Elasticsearch'),
        'Index',
        'data',
        (SELECT id FROM config_value_types WHERE name = 'string')
    );

INSERT INTO config_defaults
    (section_id, cfg_key, cfg_value, value_type_id)
VALUES
    (
        (SELECT id FROM config_sections WHERE name = 'Email'),
        'Src',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Email'),
        'Dest',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Email'),
        'PermIDRequestDest',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Email'),
        'SupportDest',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    );
