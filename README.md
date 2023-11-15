# mgmt

A tool for managing DE deployments.

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

#### Importing config values from an existing environment

### Configuration management

#### Default values

#### Environment values

#### Rendering configuration files

### `mgmt` database

#### Dolt

#### Starting up

#### Backing up

#### Restoring
