# Database Notes

## Rationale

## Goals

The goals for having the configuration values and their default tracked in a database are the following:

- Provide an intermediate format that can be serialized into other formats.
- Allow for querying of configuration values.
- Allow for querying of configuration values by which file templates use them.
- Allow for querying of configuration values by which services use them.
- Easily track changes in configuration values across environments.
- Allow users to rollback configuration changes.
- Allow users to easily back up configuration values.
- Allow users to query historical configurations.

## Schema

The schema for the database acts as the intermediate format for the configurations, as outlined in the goals section. From the schema, we can generate domain objects that can then be serialized into formats needed by the deployment systems in use, such as Terraform or skaffold.

The files for the schema are in the `mgmt/db/migrations` directory. You can also connect to the running Dolt database and browse the schema, either with the built-in Dolt client or with the mysql client.

We are tracking the following objects in the database:

- Git repositories, including their ref (i.e. branch, tag, ref)
- Services
- Container images
- Environments
- Configuration templates
- Configuration sections
- Configuration defaults
- Configuration values

We also track relationships between the objects. For instance, a service is related to a container image, which in turn is associated with repository. This allows us to track which services are affected by changes to a repository.

Another example is that a environment contains one or more services (via the `environments_services` table), which can have one or more configuration templates (via the `environments_services_config_templates` table). This allows us to detect which services are affected by changes to a configuration template.

Similarly, a service in an environment is also associated with one or more configuration values (via the `environments_services_config_values` table). This allows us to detect which services and environments are affected by a change to a configuration value.

Something to note is that configuration defaults are not constrained to an environment. They are global and the relationship between configuration defaults and configuration values are not enforced at the database level. The `cfg_key` and `cfg_value` columns in the `config_defaults` table are intended to correspond to the `cfg_key` and `cfg_value` columns in the `config_values` table. If you need to make a change to the value type or other change that is incompatible across environments, it's recommended that you branch the database until the change is available in all environments and then merge the database branch back into main/master.

## Environments

## Versioning

## Backups

## Migrations
