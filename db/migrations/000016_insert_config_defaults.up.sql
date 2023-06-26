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

INSERT INTO config_defaults
    (section_id, cfg_key, cfg_value, value_type_id)
VALUES
    (
        (SELECT id FROM config_sections WHERE name = 'Grouper'),
        'MorphString',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Grouper'),
        'Password',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Grouper'),
        'FolderNamePrefix',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Grouper'),
        'Loader.URI',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Grouper'),
        'Loader.User',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Grouper'),
        'Loader.Password',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    );

INSERT INTO config_defaults
    (section_id, cfg_key, cfg_value, value_type_id)
VALUES
    (
        (SELECT id FROM config_sections WHERE name = 'ICAT'),
        'Host',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'ICAT'),
        'Port',
        '1247',
        (SELECT id FROM config_value_types WHERE name = 'int')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'ICAT'),
        'User',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'ICAT'),
        'Password',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'ICAT'),
        'Zone',
        'iplant',
        (SELECT id FROM config_value_types WHERE name = 'string')
    );

INSERT INTO config_defaults
    (section_id, cfg_key, cfg_value, value_type_id)
VALUES
    (
        (SELECT id FROM config_sections WHERE name = 'Infosquito'),
        'DayNum',
        '4',
        (SELECT id FROM config_value_types WHERE name = 'int')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Infosquito'),
        'PrefixLength',
        '4',
        (SELECT id FROM config_value_types WHERE name = 'int')
    );

INSERT INTO config_defaults
    (section_id, cfg_key, cfg_value, value_type_id)
VALUES
    (
        (SELECT id FROM config_sections WHERE name = 'Intercom'),
        'Enabled',
        'false',
        (SELECT id FROM config_value_types WHERE name = 'bool')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Intercom'),
        'AppID',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Intercom'),
        'CompanyID',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Intercom'),
        'CompanyName',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    );

INSERT INTO config_defaults
    (section_id, cfg_key, cfg_value, value_type_id)
VALUES
    (
        (SELECT id FROM config_sections WHERE name = 'Irods'),
        'AMQP.User',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Irods'),
        'AMQP.Password',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Irods'),
        'AMQP.Host',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Irods'),
        'AMQP.Port',
        '5672',
        (SELECT id FROM config_value_types WHERE name = 'int')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Irods'),
        'AMQP.Vhost',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Irods'),
        'Host',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Irods'),
        'User',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Irods'),
        'Password',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Irods'),
        'Zone',
        'iplant',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Irods'),
        'Port',
        '1247',
        (SELECT id FROM config_value_types WHERE name = 'int')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Irods'),
        'AdminUsers',
        'rodsadmin',
        (SELECT id FROM config_value_types WHERE name = 'csv')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Irods'),
        'PermsFilter',
        'rodsadmin',
        (SELECT id FROM config_value_types WHERE name = 'csv')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Irods'),
        'ExternalHost',
        'data.cyverse.rocks',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Irods'),
        'QuotaRootResources',
        'mainIngestRes,mainReplRes',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Irods'),
        'WebDav.AnonURI',
        'https://data.cyverse.rocks/dav-anon',
        (SELECT id FROM config_value_types WHERE name = 'string')
    );

INSERT INTO config_defaults
    (section_id, cfg_key, cfg_value, value_type_id)
VALUES
    (
        (SELECT id FROM config_sections WHERE name = 'Keycloak'),
        'ServerURI',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Keycloak'),
        'Realm',
        'CyVerse',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Keycloak'),
        'ClientID',
        'de',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Keycloak'),
        'ClientSecret',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Keycloak'),
        'VICE.ClientID',
        'de-vice',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Keycloak'),
        'VICE.ClientSecret',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    );

INSERT INTO config_defaults
    (section_id, cfg_key, cfg_value, value_type_id)
VALUES
    (
        (SELECT id FROM config_sections WHERE name = 'Jobs'),
        'DataTransferImage',
        'harbor.cyverse.org/de/porklock',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'PGP'),
        'KeyPassword',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    );

INSERT INTO config_defaults
    (section_id, cfg_key, cfg_value, value_type_id)
VALUES
    (
        (SELECT id FROM config_sections WHERE name = 'PermanentID'),
        'CuratorsGroup',
        'data-curators',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'PermanentID'),
        'DataCite.BaseURI',
        'https://api.datacite.org',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'PermanentID'),
        'DataCite.User',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'PermanentID'),
        'DataCite.Password',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'PermanentID'),
        'DOIPrefix',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    );

INSERT INTO config_defaults
    (section_id, cfg_key, cfg_value, value_type_id)
VALUES
    (
        (SELECT id FROM config_sections WHERE name = 'Unleash'),
        'BaseURL',
        'http://unleash:4242',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Unleash'),
        'APIPath',
        '/api',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Unleash'),
        'MaintenanceFlag',
        'DE-Maintenance',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Unleash'),
        'APIToken',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    );

INSERT INTO config_defaults
    (section_id, cfg_key, cfg_value, value_type_id)
VALUES
    (
        (SELECT id FROM config_sections WHERE name = 'UserPortal'),
        'BaseURI',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Admin'),
        'Groups',
        'de_admins',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Admin'),
        'Attribute',
        'entitlement',
        (SELECT id FROM config_value_types WHERE name = 'string')
    );

INSERT INTO config_defaults
    (section_id, cfg_key, cfg_value, value_type_id)
VALUES
    (
        (SELECT id FROM config_sections WHERE name = 'Analytics'),
        'Enabled',
        'false',
        (SELECT id FROM config_value_types WHERE name = 'bool')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Analytics'),
        'Id',
        'g-id',
        (SELECT id FROM config_value_types WHERE name = 'string')
    );

INSERT INTO config_defaults
    (section_id, cfg_key, cfg_value, value_type_id)
VALUES
    (
        (SELECT id FROM config_sections WHERE name = 'Harbor'),
        'URL',
        'harbor.cyverse.org',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Harbor'),
        'ProjectQARobotName',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    ),
    (
        (SELECT id FROM config_sections WHERE name = 'Harbor'),
        'ProjectQARobotSecret',
        '',
        (SELECT id FROM config_value_types WHERE name = 'string')
    );
