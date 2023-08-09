# 0. Overview

This file covers how configuration values are organized in the `de_releases` database used by the `mgmt` tool. Included topics are
* How everything is organized.
* How to add new configuration values.
* How to add new configuration section.
* How to make manage changes to the `de_releases` database.

&nbsp;

# 1. Organization

The configuration values are stored in a [`dolt`](http://dolthub.com) database. The initial creation and population of the database was done with a combination of the `dolt` command and the [`migrate`](https://github.com/golang-migrate/migrate) command. The migration SQL files are stored in the [`db/migrations`](../db/migrations) directory. For future updates, only add migration files if you're changing the schema of the database. Adding new data to the database can be done through the `mgmt` tool or through ad-hoc SQL queries through `dolt` or `mysql`.

An `environment` is a single deployment of a DE. It has a name and a corresponding namespace. The namespace can be different from the name. A new environment can be set with the `mgmt-configs env create` command.

Each configuration value is contained in a top-level `section`. New sections can be added with the `mgmt-configs sections add` commmand.

Every configuration also has a corresponding default value. Default values are global and therefore reused across environments. New default values can either be added as part of a migration or by using the `mgmt-config defaults set` command.

In summary, an environment contains multiple configuration sections, each of which contain a set of configuration values. Each configuration has a globally defined default.

&nbsp;

# 2. Environments

A environment is meant to represent a single deployment of the DE. A site may have one or more deployments, and therefore have one or more corresponding environments. An example would be a site having a `qa` and `prod` environment, with new changes being tried out in the `qa` environment before being deployed into `prod`.

## 2.1 Listing available environments

Use the `mgmt-configs env list` command to get a simple listing of all of the environments defined in the database.

Example:
```bash
> mgmt-configs env list
de
testenv
qa
```

## 2.2 Adding a new environment

Use the `mgmt-configs env create` command to add a new enviroment.

Example:
```bash
> mgmt-configs env create --name qa --namespace qa
Created environment: qa
```

## 2.3 Deleting an environment

Use the `mgmt-configs env delete` command to delete an environment.

Example:
```bash
> mgmt-configs env delete --name qa
```

## 2.4 Interactively populating all configuration values in an environment

This command will go through all of the configuration values, section by section, and allow you to set values for them.

```bash
> mgmt-configs env populate
```

The output of this command is long, so it's not included here.

&nbsp;

# 3. Sections

Each section contains a set of configuration values in an environment. The list of sections is global, but the values contained within them are per-environment.

## 3.1 Listing sections

Use the `mgmt-configs sections list` command to list all of the available sections.

```bash
> mgmt-configs sections list
```

## 3.2 Adding a new section

Use the `mgmt-configs sections add` command to add a new section to the database.

```bash
> mgmt-configs sections add --section Example
```

## 3.3 Deleting a section

Use the `mgmt-configs sections delete` command to delete a section from the database.

```bash
> mgmt-configs sections delete --section Example
```

&nbsp;

# 4.0 Default Values

Default values are global (as are sections), which means they're available for every environment.

## 4.1 Listing default values

Use the `mgmt-configs defaults list` command to list all of the available default values.

```bash
> mgmt-configs defaults list
Admin.Attribute = entitlement
Admin.Groups = de_admins
Agave.CallbackBaseURI =
...
```

Note that some of the defaults will be blank. That is by design. Those values should be overridden by the environment-specific configuration values.

## 4.2 Adding a default value

Use the `mgmt-configs defaults set` command to add a new default value.

```bash
> mgmt-configs defaults set --section Example --key Middle.Key --value ExampleValue --type string
Added default config value with an ID of 165
```

## 4.3 Getting a single default value

Use the `mgmt-configs defaults get` command to get a single default value.

```bash
> mgmt-configs defaults get --section Example --key Middle.Key
Example.Middle.Key = ExampleValue
```

## 4.4 Deleting a default value

Use the `mgmt-configs defaults delete` command to delete a default value.

```bash
> mgmt-configs defaults delete --section Example --key Middle.Key
Deleted default config value with an ID of 165
```

&nbsp;

# 5. Configuration Values

Configuration values are environment specific and override a default value. A default value with the same section and key must be present in order for the value to be set.

## 5.1 Listing configuration values

Use the `mgmt-configs values list` command to list all of the configuration values in an environment.

```bash
> mgmt-configs values list
TopLevel.Environment = qa
TopLevel.Namespace = qa
...
```

## 5.2 Adding a configuration value

Use the `mgmt-configs values set` command to set a new configuration value that overrides a default.

```bash
> mgmt-configs values set -e qa -s Example -k Middle.Key -v ExampleValue -t string
Added config value to environment 'qa': Exmaple.Middle.Key = ExampleValue
```

## 5.3 Getting a single configuration value

Use the `mgmt-configs values get` command to get a configuration value.

```bash
> mgmt-configs values get -e qa -s Example -k Middle.Key
Example.Middle.Key = ExampleValue
```

## 5.4 Deleting a configuration value

Use the `mgmt-configs values delete` command to delete a configuration value.

```bash
> mgmt-configs values delete -e qa -s Example -k Middle.Key
Deleted config value from environment 'qa': Exmaple.Middle.Key
```

&nbsp;

# 6. YAML rendering

Rendering (a.k.a applying, a.k.a generating) YAML files containing the configuration values for an environment allows the operator to use the configuration values as part of the DE deployment process. They're usually provided to the existing `deploy.py` script --- along with a YAML file containing the default values --- to allow it to generate configuration files for the services.

To "render" the YAML file containing the configuration values for an environment, use the `mgmt-configs values render` command.

```bash
> mgmt-configs values render -e qa --include-all
<YAML>
```

The `mgmt-configs values render` command has a lot of flags to control which optional top-level sections are included in the YAML output. By default, all optional sections are disabled. The `--include-all` option is a short-hand way to enable all of the optional sections. See the output of the `mgmt-configs values render --help` command to get a full listing of all of the sections that can be enabled.

&nbsp;

# 7. Database

`mgmt` uses a Dolt database to store all of the information needed to generate the configuration values files for an environment. Dolt is versioned, so you can do a `dolt commit` after updating a value to ensure that you can roll back a change if necessary. For more information on Dolt, check out their documentation at [https://dolthub.com](https://dolthub.com).

## 7.1 Creating the data directory

First you need to create the directory where the database files will live. Here's the process for that, run from inside this repo's top-level directory:

```bash
> cd db/
> mkdir de_releases
> cd de_releases/
> dolt init
```

## 7.2 Starting Up

To start up the database, run the `dolt sql-server` in the `db/de_releases` directory. If 

```bash
> cd db/de_releases
> dolt sql-server
```

## 7.3 Connecting

You have two options for directly connecting to the Dolt database. You can use the `dolt sql-client` command or the `mysql` client. Both work well.

For the `dolt sql-client` command, run the following while `dolt sql-server` is running in another terminal:

```bash
> dolt sql-client -u root
```

For the MySQL command, run the following while `dolt sql-server` is running in another terminal:

```bash
> mysql --host 127.0.0.1 -u root --port 3306
```

## 7.4 Creating the database and running migrations

Next you need to create the database. For that you'll need a prompt as created in the previous step. Then you'll need to run the following:

```bash
mysql> create database de_releases;
mysql> use de_releases;
```

 The latter command, `use de_releases;`, is one that you'll have to run every time you start up a REPL connected to the dolt server.

 Next you should run the migrations against the database while `dolt sql-server` is still running in another terminal. To do that, exit the database REPL with `Ctrl-D` and run the following from the top-level directory of this repository.

 ```bash
> migrate -database 'mysql://root@tcp(127.0.0.1:3306)/de_releases' -path db/migrations up
```

## 7.5 Backup and restore
## 7.6 Reusing across clusters

&nbsp;

# Sources
* [Dolt](https://dolthub.com)
* [migrate](https://github.com/golang-migrate/migrate)
* [Migration SQL)(../db/migrations) 
