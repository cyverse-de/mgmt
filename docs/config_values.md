# 0. Overview

This file covers how configuration values are organized in the `de_releases` database used by the `mgmt` tool. Included topics are
* How everything is organized.
* How to add new configuration values.
* How to add new configuration section.
* How to make manage changes to the `de_releases` database.

&nbsp;

# 1. Organization

The configuration values are stored in a [`dolt`](http://dolthub.com) database. The initial creation and population of the database was done with a combination of the `dolt` command and the [`migrate`](https://github.com/golang-migrate/migrate) command. The migration SQL files are stored in the [`db/migrations`](../db/migrations) directory. For future updates, only add migration files if you're changing the schema of the database. Adding new data to the database can be done through the `mgmt` tool or through ad-hoc SQL queries through `dolt` or `mysql`.

All configuration values have a default value. The defaults are environment agnostic, meaning that the default value should potentially work for all deployments or be blank. New default values can either be added as part of a migration or by using the `mgmt-config defaults set` command.

&nbsp;

# 2. Environments
## 2.1 Listing available environments
## 2.2 Adding a new environment
## 2.3 Deleting an environment

&nbsp;

# 3. Sections
## 3.1 Listing sections
## 3.2 Adding a new section
## 3.3 Listing a single section
## 3.4 Deleting a section

&nbsp;

# 4. Default Values
## 4.1 Listing default values
## 4.2 Adding a default value
## 4.3 Getting a single default value
## 4.4 Deleting a default value

&nbsp;

# 5. Configuration Values
## 5.1 Listing configuration values
## 5.2 Adding a configuration value
## 5.3 Getting a single configuration value
## 5.4 Deleting a configuration value

&nbsp;

# 6. YAML rendering
# 6.1 Adding a new configuration value
# 6.2 Adding a new section
# 6.3 Updating the database

&nbsp;

# 7. Database
## 7.1 Schema
## 7.2 Versioning
## 7.3 Migrations
## 7.4 Backup & Restore
## 7.5 Reusing across clusters

&nbsp;

# Sources
* [Dolt](https://dolthub.com)
* [migrate](https://github.com/golang-migrate/migrate)
* [Migration SQL)(../db/migration) 
