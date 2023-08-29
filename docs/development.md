# Development Notes for `mgmt`

## Adding an environment

New code changes should not be needed for adding a new environment. It's recommended that you use the `mgmt-configs env create`command to add a new environment to the database.

## Adding a configuration section as a domain type

Code changes will need to be added to fully support adding a new configuration section.

If the section is a top-level section containing other values, you'll need to either add a new `.rs` file to the `src/config_values` directory containing the code for the section, or add the section to an existing file in the same location.

Each section in the configuration needs to have it's own Rust struct that acts as a domain type. For example, the `Grouper` section of the config has a struct in `src/config_values/grouper.rs` that looks like the following:

```rust
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Grouper {
    #[serde(skip)]
    section: String,
    morph_string: String,
    password: String,
    folder_name_prefix: String,
    loader: GrouperLoader,
}
```

The `#[derive(Serialize, Deserialize, Clone)]` part is an attribute macro telling the Rust compiler to generate code that implements the Serialize, Deserialize, and Clone traits for the `Grouper` type. Similarly, the `#[serde(rename_all = "PascalCase")]` attribute macro tells the `serde` crate to rename the fields in the `Grouper` struct when serializing or deserialing it so that their names conform to the Pascal capitalization standard in the emitted representation. You will want to include that for all section structs that you add to the codebase.

The `#[serde(skip)]` field macro over `section: String` tells the `serde` package to skip that field when reading or writing a `Grouper` section out. You'll want to add a `section` field to every config section struct that you add to `mgmt` and you'll want to make sure that it's ignored when writing out the YAML files, so make sure that field macro is included.

`morph_string`, `password`, and `folder_name_prefix` are configuration values contained within the `Grouper` section of the config. We'll cover more on adding configuration values in a later section, so we'll leave it at that for now. Note that the `#[serde(skip)]` macro is missing from these fields, which means that they are serialized and deserialized to/from the emitted representation. Also worth noting is that the `#[serde(rename_all = "PascalCase")]` macro does apply to each of the fields in the struct, so `folder_name_prefix` will turn into `FolderNamePrefix` in any representations of the struct created by `serde`.

The `loader` field is an example of a nested subject with a type of `GrouperLoader`. Subjects nested inside another subject do not need to have their own entry in the `de_releases` database, but they do need to be represented in the codebase so the `serde` crate knows what to do with the values contained within them. The struct for the `GrouperLoader` type looks like this:

```rust
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GrouperLoader {
    #[serde(skip)]
    section: String,

    #[serde(rename = "URI")]
    uri: Option<Url>,

    user: String,
    password: String,
}
```

All of the same rules for adding a configuration section that were outlined above still apply to a nested section. Some changes to note here are the use of the `#[serde(rename = "URI")]` macro over the `uri` field and the fact that the same field has a type of `Option<Url>`. The `Url` type comes from the `url` module imported at the top of the file (but not shown here). The fact that it's an `Option` means that the field is optional and will be represented as a null value in the serialization output if no non-None value is set as the default and the user does not supply another value.

The generated YAML from the combination of `Grouper` and `GrouperLoader` will result in output that looks like the following:

```yaml
Grouper:
  MorphString: asdfasdfasdfa
  Password: this-is-fake
  FolderNamePrefix: cyverse:de
  Loader:
    URI: http://grouper-loader-example
    User: a-user
    Password: a-password
```

Notice that the name of the `Loader` object comes from the corresponding field name in the `Grouper` type, not from the name of the `GrouperLoader` type.

Finally, add a your new configuration section to the `ConfigValues` type in the `src/config_values/çonfig.rs` file. The `ConfigValues` type represents the configuration as a whole, and you need to add your new section as a field to its struct in order for it to get recognized when working with the configuration later.

Here's a truncated version of the `ConfigValues` struct that shows the `grouper` section being added to it.

```rust
use crate::config_values::grouper::Grouper;
...

pub struct ConfigValues {
  ...
  grouper: Grouper,
  ...
}
```

The remaining sections of the document will build off of this foundation to support the rest of the features needed from a configuration section in the `mgmt` tool.

## Adding a new default config value

To add support for a new configuration value, you need to implement the `Default` trait for the type you added for the configuration section. For example, for the `Grouper` section, the `Default` trait implementation looks like this:

```rust
impl Default for Grouper {
    fn default() -> Self {
        Grouper {
            section: "Grouper".to_string(),
            morph_string: String::new(),
            password: String::new(),
            folder_name_prefix: String::new(),
            loader: GrouperLoader::default(),
        }
    }
}
```

The interesting parts here are to make sure that the default for the `section` field matches what the field will look like in the configuration files themselves. For instance here it's set to `Grouper` since the top-level section name is `Grouper`. The `morph_string`, `password`, and `folder_name_prefix` fields have empty strings as their defaults, which means that the serialized configuration will have an empty value (as opposed to null with an unset `Option`) unless the user provides an overriding config value for the field.

The `loader` field has a default value that delegates to GrouperLoader's implementation of the `Default` trait. Here's what that looks like:

```rust
impl Default for GrouperLoader {
    fn default() -> Self {
        GrouperLoader {
            section: "Grouper".to_string(),
            uri: None,
            user: String::new(),
            password: String::new(),
        }
    }
}
```

There's a couple of things to note here:

1. The section is still set to `Grouper` as opposed to `GrouperLoader`.
2. The default value for the `uri` field is `None`.

Since `GrouperLoader` is the struct for a nested section, its corresponding `section` field should have a default of the name of the outermost containing section --- in this case, `Grouper`. That should be the case regardless of how deeply nested the section is. The default value for the `uri` field is `None` because the field is defined as an `Option<Url>`.

## Loading domain objects from the database.

Loading domain objects (i.e. configuration sections) from the database allows us to write out the configuration values as YAML files. This works by:

- Querying the database for the configuration values (or the defaults if the values are unset).
- Creating new domain objects from the types in the `config_values` directory.
- Loading the domain objects up with the results of the queries.
- Using the `serde` crate to write out the domain objects into a YAML file.

Most of this process is fairly generic or implemented at a more abstract level, so you'll just need to implement the `LoadFromDatabase` trait for each domain type that turns a `Vec<db::ConfigurationValue>` into a domain object.

Here's how the `LoadFromDatabase` trait looks for the `Grouper` domain type (which corresponds to the `Grouper` section):

```rust
use crate::db::{self, add_env_cfg_value, set_config_value, LoadFromDatabase};

impl LoadFromDatabase for Grouper {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "MorphString" => self.morph_string = value,
                "Password" => self.password = value,
                "FolderNamePrefix" => self.folder_name_prefix = value,
                _ => (),
            }

            if key.starts_with("Loader.") {
                self.loader.cfg_set_key(cfg)?;
            }
        }
        Ok(())
    }
}
```

The `get_section` method allows callers to make decisions based on what section the domain object applies to. This kind of logic is implemented sparingly, but it does exist, so the function is needed.

The `cfg_set_key` method is required to be implemented for each domain type. It's responsible for mapping a key to a field in the type. Note the logic for checking if the key starts with `Loader.`: that code snippet is repeated a lot when a field on a domain type has a type that corresponds to another domain type. In other words, that's how you handle a configuration section having a nested configuration section.

That code snippet delegates to `GrouperLoader`'s implementation of the `LoadFromDatabase` trait, which looks like this:

```rust
impl LoadFromDatabase for GrouperLoader {
    fn get_section(&self) -> String {
        self.section.to_string()
    }

    fn cfg_set_key(&mut self, cfg: &crate::db::ConfigurationValue) -> anyhow::Result<()> {
        if let (Some(key), Some(value)) = (cfg.key.clone(), cfg.value.clone()) {
            match key.as_str() {
                "Loader.URI" => self.uri = Url::parse(&value).ok(),
                "Loader.User" => self.user = value,
                "Loader.Password" => self.password = value,
                _ => (),
            }
        }
        Ok(())
    }
}
```

The logic here is largely the same, though it's worth noting that the keys the `match` statement is checked against are all prefixed with the name of the subsection. If the subsection itself contained a subsection, that doubly-nested subsection's keys would look like `Loader.DoubleNested.FieldName`.

## Serializing domain objects to the database.

Writing out domain objects to the database allows us to import existing config values files into the database. This works by:

- Reading in the files and deserializing them to a `ConfigValues` domain object, as defined in `src/config_values/config.rs`. This will cause all of the other domain objects to get created an populated.
- Turning each of the domain objects into a vector of ConfigurationValues.
- Iterating through all of the ConfigurationValues and writing them out to the database.

As with loading domain objects from the database, most of this logic is fairly generic or implemented at an abstract level, so you'll just need to implement a trait for each new domain type (i.e. configuration section) that can turn an instance of it into a `Vec<db::ConfigurationValue>`.

## Supporting user prompts for a new value
