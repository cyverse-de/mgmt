use crate::{
    config_values::{self, config::ConfigValues},
    db,
};
use anyhow::Context;
use base64::{engine::general_purpose, Engine as _};
use sqlx::{MySql, Transaction};
use std::{collections::HashMap, fs, path::PathBuf};
use tera::{to_value, try_get_value, Result as TeraResult, Tera, Value};

/// A custom filter for Tera that base64 encodes a Value.
fn base64_encode(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let s = try_get_value!("base64_encode", "value", String, value);
    let encoded: String = general_purpose::STANDARD.encode(s.as_bytes());

    Ok(to_value(encoded).unwrap())
}

/// Creates a new Tera instance with the base64_encode filter registered.
fn new_tera() -> Tera {
    let mut tera = Tera::default();
    tera.register_filter("base64_encode", base64_encode);
    tera
}

/// Creates a new Tera instance with the base64_encode filter registered and
/// templates loaded from a directory.
fn new_tera_dir(templates_path: &PathBuf) -> anyhow::Result<Tera> {
    let mut tera = Tera::new(
        templates_path
            .to_str()
            .context("failed to get the templates path")
            .unwrap(),
    )?;
    tera.register_filter("base64_encode", base64_encode);
    Ok(tera)
}

// Internal function that renders the values out to a file.
fn render_t(
    template_path: &PathBuf,
    defaults_values: &ConfigValues,
    env_values: &ConfigValues,
    out_path: &PathBuf,
) -> anyhow::Result<()> {
    let mut defaults_context = tera::Context::from_serialize(defaults_values)?;
    let values_context: tera::Context = tera::Context::from_serialize(env_values)?;

    defaults_context.extend(values_context);

    let out_file = fs::File::create(out_path)?;
    let mut tera = new_tera();
    tera.add_raw_template("template", &fs::read_to_string(template_path)?)?;

    Ok(tera.render_to("template", &defaults_context, out_file)?)
}

// Internal function that renders a directory of templates out to a directory.
fn render_d(
    templates_path: &PathBuf,
    defaults_values: &ConfigValues,
    env_values: &ConfigValues,
    out_path: &PathBuf,
) -> anyhow::Result<()> {
    let mut defaults_context = tera::Context::from_serialize(defaults_values)?;
    let values_context: tera::Context = tera::Context::from_serialize(env_values)?;

    defaults_context.extend(values_context);

    let tera = new_tera_dir(templates_path)?;

    for name in tera.get_template_names() {
        let out_file = out_path.join(name);
        let out_writer = fs::File::create(&out_file)?;
        tera.render_to(name, &defaults_context, out_writer)?;
    }

    Ok(())
}

/// Renders a template out to a file. Uses the defaults and values files to
/// populate the template.
pub fn render_template(
    template_path: &PathBuf,
    defaults_path: &PathBuf,
    values_path: &PathBuf,
    out_path: &PathBuf,
) -> anyhow::Result<()> {
    let defaults_file = fs::File::open(defaults_path)?;
    let defaults_values: config_values::config::ConfigValues =
        serde_yaml::from_reader(defaults_file)?;

    let values_file = fs::File::open(values_path)?;
    let values: config_values::config::ConfigValues = serde_yaml::from_reader(values_file)?;

    Ok(render_t(
        template_path,
        &defaults_values,
        &values,
        out_path,
    )?)
}

/// Renders a template out to a file, using the defaults and values queried
/// from the database for the provided environment to populate the template.
pub async fn render_template_from_db(
    tx: &mut Transaction<'_, MySql>,
    template_path: &PathBuf,
    env: &str,
    out_path: &PathBuf,
) -> anyhow::Result<()> {
    let default_values: ConfigValues = db::list_default_config_values(tx, None, None).await?.into();
    let env_values: ConfigValues = db::list_config_values(tx, Some(env), None, None)
        .await?
        .into();

    Ok(render_t(
        template_path,
        &default_values,
        &env_values,
        out_path,
    )?)
}

/// Renders a directory of templates out to a directory. Uses the defaults and
/// values files to populate the templates.
pub fn render_template_dir(
    templates_path: &PathBuf,
    defaults_path: &PathBuf,
    values_path: &PathBuf,
    out_path: &PathBuf,
) -> anyhow::Result<()> {
    let defaults_file = fs::File::open(defaults_path)?;
    let defaults_values: config_values::config::ConfigValues =
        serde_yaml::from_reader(defaults_file)?;

    let values_file = fs::File::open(values_path)?;
    let values: config_values::config::ConfigValues = serde_yaml::from_reader(values_file)?;

    Ok(render_d(
        templates_path,
        &defaults_values,
        &values,
        out_path,
    )?)
}

/// Renders a directory of templates out to a directory, using the defaults and
/// values queried from the database for the provided environment to populate
/// the templates.
pub async fn render_template_dir_from_db(
    tx: &mut Transaction<'_, MySql>,
    templates_path: &PathBuf,
    env: &str,
    out_path: &PathBuf,
) -> anyhow::Result<()> {
    let default_values: ConfigValues = db::list_default_config_values(tx, None, None).await?.into();
    let env_values: ConfigValues = db::list_config_values(tx, Some(env), None, None)
        .await?
        .into();

    Ok(render_d(
        templates_path,
        &default_values,
        &env_values,
        out_path,
    )?)
}
