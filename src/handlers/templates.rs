use crate::{
    config_values::config::{ConfigValues, SectionOptions},
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
    let merged_cv = defaults_values.merge_with(&env_values)?;
    let defaults_context = tera::Context::from_serialize(merged_cv)?;

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
    let merged_cv = defaults_values.merge_with(&env_values)?;
    let defaults_context = tera::Context::from_serialize(&merged_cv)?;

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
    let mut default_values: ConfigValues = serde_yaml::from_reader(defaults_file)?;
    default_values.set_section_options(default_values.generate_section_options());

    let values_file = fs::File::open(values_path)?;
    let mut values: ConfigValues = serde_yaml::from_reader(values_file)?;
    values.set_section_options(values.generate_section_options());

    Ok(render_t(template_path, &default_values, &values, out_path)?)
}

/// Renders a template out to a file, using the defaults and values queried
/// from the database for the provided environment to populate the template.
pub async fn render_template_from_db(
    tx: &mut Transaction<'_, MySql>,
    template_path: &PathBuf,
    env: &str,
    out_path: &PathBuf,
) -> anyhow::Result<()> {
    let mut default_values: ConfigValues =
        db::list_default_config_values(tx, None, None).await?.into();
    default_values.set_section_options(default_values.generate_section_options());

    let mut env_values: ConfigValues = db::list_config_values(tx, Some(env), None, None)
        .await?
        .into();
    let section_options: SectionOptions = db::get_feature_flags(tx, env).await?.into();
    env_values.set_section_options(section_options);

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
    let mut defaults_values: ConfigValues = serde_yaml::from_reader(defaults_file)?;
    defaults_values.set_section_options(defaults_values.generate_section_options());

    let values_file = fs::File::open(values_path)?;
    let mut values: ConfigValues = serde_yaml::from_reader(values_file)?;
    values.set_section_options(values.generate_section_options());

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
    let default_values_list: Vec<db::ConfigurationValue> =
        db::list_default_config_values(tx, None, None).await?;
    let mut default_values: ConfigValues = default_values_list.into();
    default_values.set_section_options(default_values.generate_section_options());

    let mut env_values: ConfigValues = db::list_config_values(tx, Some(env), None, None)
        .await?
        .into();
    let section_options: SectionOptions = db::get_feature_flags(tx, env).await?.into();
    env_values.set_section_options(section_options);

    Ok(render_d(
        templates_path,
        &default_values,
        &env_values,
        out_path,
    )?)
}

/// Renders templates returned from the database with values returned
/// from the database. It's database all the way down.
pub async fn render_db(
    tx: &mut Transaction<'_, MySql>,
    env: &str,
    templates_dir: &PathBuf,
    out_path: &PathBuf,
) -> anyhow::Result<()> {
    println!("Rendering templates from values in the database.");

    println!("Getting template paths from the database...");
    let template_paths = db::list_templates(tx, &env).await?;

    println!("Getting values from the database...");
    let mut default_values: ConfigValues =
        db::list_default_config_values(tx, None, None).await?.into();
    default_values.set_section_options(default_values.generate_section_options());

    let mut env_values: ConfigValues = db::list_config_values(tx, Some(env), None, None)
        .await?
        .into();
    let section_options: SectionOptions = db::get_feature_flags(tx, env).await?.into();
    env_values.set_section_options(section_options);

    default_values = default_values.merge_with(&env_values)?;

    println!("{:#?}", env_values);

    println!("Merging defaults and values...");
    let defaults_context = tera::Context::from_serialize(default_values)?;

    let mut tera = new_tera();
    for template_path in template_paths {
        let tmpl_path = PathBuf::from(&template_path);
        let mut out_dir = out_path
            .parent()
            .context("failed to get the parent directory")?
            .to_path_buf();

        // Make sure the output directory doesn't contain a template directory
        // since that gets really confusing when inspecting the output.
        if let Some(output_dir) = out_dir.file_name() {
            if output_dir == "templates" {
                out_dir = out_dir
                    .parent()
                    .context("failed to get the parent directory")?
                    .to_path_buf();
            }
        }

        // Make sure the environment sub directory is appended to the output
        // directory, so that configs for different environments don't
        // overwrite each other.
        if let Some(env_dir) = out_dir.file_name() {
            if env_dir != env {
                out_dir = out_dir.join(env);
            }
        }

        let output_filename = tmpl_path
            .file_name()
            .context("failed to get the filename")?
            .to_str()
            .context("failed to convert the filename to a string")?;
        let out_file = out_dir.join(&output_filename);

        println!("Creating {}...", out_dir.display());
        fs::create_dir_all(out_dir)?;

        println!("Creating {}...", out_file.display());
        let out_writer = fs::File::create(&out_file)?;

        let out_file_str = out_file
            .to_str()
            .context("failed to get the output file path")?;
        let full_template_path = templates_dir.join(&template_path);
        println!(
            "Rendering {} from template {}...",
            out_file.display(),
            full_template_path.display()
        );
        tera.add_raw_template(&out_file_str, &fs::read_to_string(&full_template_path)?)?;
        tera.render_to(&out_file_str, &defaults_context, out_writer)?;
    }

    Ok(())
}
