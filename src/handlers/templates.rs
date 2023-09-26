use crate::config_values;
use anyhow::{Context, Result};
use std::{fs, path::PathBuf};
use tera::Tera;

/// Renders a template out to a file. Uses the defaults and values files to
/// populate the template.
pub fn render_template(
    template_path: &PathBuf,
    defaults_path: &PathBuf,
    values_path: &PathBuf,
    out_path: &PathBuf,
) -> Result<()> {
    let defaults_file = fs::File::open(defaults_path)?;
    let defaults_values: config_values::config::ConfigValues =
        serde_yaml::from_reader(defaults_file)?;
    let mut defaults_context = tera::Context::from_serialize(defaults_values)?;

    let values_file = fs::File::open(values_path)?;
    let values: config_values::config::ConfigValues = serde_yaml::from_reader(values_file)?;
    let values_context: tera::Context = tera::Context::from_serialize(values)?;

    defaults_context.extend(values_context);

    let out_file = fs::File::create(out_path)?;
    let mut tera = Tera::default();
    tera.add_raw_template("template", &fs::read_to_string(template_path)?)?;

    Ok(tera.render_to("template", &defaults_context, out_file)?)
}

pub fn render_template_dir(
    templates_path: &PathBuf,
    defaults_path: &PathBuf,
    values_path: &PathBuf,
    out_path: &PathBuf,
) -> Result<()> {
    let defaults_file = fs::File::open(defaults_path)?;
    let defaults_values: config_values::config::ConfigValues =
        serde_yaml::from_reader(defaults_file)?;
    let mut defaults_context = tera::Context::from_serialize(defaults_values)?;

    let values_file = fs::File::open(values_path)?;
    let values: config_values::config::ConfigValues = serde_yaml::from_reader(values_file)?;
    let values_context: tera::Context = tera::Context::from_serialize(values)?;

    defaults_context.extend(values_context);

    let tera = Tera::new(
        templates_path
            .to_str()
            .context("failed to get the templates path")?,
    )?;

    for name in tera.get_template_names() {
        let out_file = out_path.join(name);
        let out_writer = fs::File::create(&out_file)?;
        tera.render_to(name, &defaults_context, out_writer)?;
    }

    Ok(())
}
