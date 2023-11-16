# mgmt

A tool for managing DE deployments.

## Overview

`mgmt` is a tool used to manage the configuration of the CyVerse Discovery Environment. It allows users to have multiple sites consisting of one or more environments. Each environment corresponds to a deployment of the Discovery Environment. A set of configuration values (a.k.a "config values", or simply "values") is set up for each environment. Each config value overrides a default value (a.k.a "default"), all of which are global and not specific to a site or environment.

Each config default and value have a section, key, value, and value type. A section corresponds to a top-level object in a YAML file or a section in an INI file. The key is remainder of the selector in a YAML file. For example if the YAML selector for a value is `DE.BaseURL.Protocol`, the section is `DE` and the key is `BaseURL.Protocol`. The value is the what the config value contains and the type tells mgmt what format to expect the value to be in.

The configuration defaults and values for a site are stored in a Dolt database, which is a MySQL-compatible versioned database.

## Definitions

## Development

### Install Rust and Cargo on MacOS with zsh

Instructions originally found at https://stackoverflow.com/a/68617314.

Run the following in a terminal.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

When prompted, customize the installation. When asked about modifying the PATH,
select `No`. All of the other questions can be set to their default setting.

Next, open your `.zshrc` file (typically located at `~/.zshrc`) and add:

```
export PATH PATH="$HOME/.cargo/bin:$PATH"
```

Finally, source your `.zshrc` file:\

```
. ~/.zshrc
```

Run `rustc --version` to make sure the rust compiler is installed.

Run `cargo --version` to make sure that cargo is installed.

## Usage

### Setting up a site

A site is a collection of environments, with each environment being its own complete Discovery Environment installation. Each site is just a separate instance of the de_releases Dolt database. As such, to set up a new site you need to clone the de_releases database.

#### Clone the de_releases database

To clone the de_releases dolt database, do the following:

```
dolt clone johnworth/de_releases <site-name>
```

For the purposes of this document, we'll use the name 'tutorial_site' for the site. This means that the above command will look like this:
```
dolt clone johnworth/de_releases tutorial_site
```

That will create a directory called `tutorial_site`. To start the database, run the following from inside the newly created directory:

```
dolt sql-server
```

To connect to the database run the following in another terminal while `dolt sql-server` is still running:

```
dolt sql-client -u root
```

Then in the SQL prompt, enter:
```
use tutorial_site;
```

### Creating a environment for the site

There are two methods for setting up the environment:
* The interactive `mgmt env populate` command.
* The `mgmt env create` command.

Of the two, the `mgmt env populate` is the recommended method.

#### Interactively populating an environment

To interactively create and populate an environment run the following command:

```
mgmt --database-url "mysql://root@127.0.0.1:3306/tutorial_site" env populate
```

Fill in the information as prompted. The database will not be updated until you successfully fill in the information (or use the defaults). The environment should not exist before running the command.

#### Importing config values from an existing environment

To import config values from existing defaults and values YAML files for an environment, run the following command:

```
mgmt --database-url "mysql://root@127.0.0.1:3306/tutorial_site" configs defaults import --file <path-to-defaults.yaml> --environment <env-name>
```

Followed by the following to import the config values:

```
mgmt --database-url "mysql://root@127.0.0.1:3306/tutorial_site configs values import --file <path-to-values.yaml> --environment <env-name>
```

### Configuration management

#### Default values

To create or set a new default value, run a command like the following:
```
mgmt --database-url "mysql://root@127.0.0.1:3306/tutorial_site configs defaults set -s <section> -k <key> -v <value> -t <type>
```

Please note that the section must exist first. See the instructions for `Sections` for more info.

#### Sections

To see the list of available sections:
```
mgmt --database-url "mysql://root@127.0.0.1:3306/tutorial_site configs sections list
```

To add a new section:
```
mgmt --database-url "mysql://root@127.0.0.1:3306/tutorial_site configs sections add --section <section-name>
```

#### Environment values

#### Rendering configuration files

### `mgmt` database

#### Dolt

#### Starting up

#### Backing up

#### Restoring
