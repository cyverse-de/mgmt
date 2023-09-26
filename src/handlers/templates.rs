use crate::config_values;
use anyhow::Result;
use std::{fs, path::PathBuf};
use tera::{Context, Tera};

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
    let mut defaults_context = Context::from_serialize(defaults_values)?;

    let values_file = fs::File::open(values_path)?;
    let values: config_values::config::ConfigValues = serde_yaml::from_reader(values_file)?;
    let values_context: Context = Context::from_serialize(values)?;

    defaults_context.extend(values_context);

    let out_file = fs::File::create(out_path)?;
    let mut tera = Tera::default();
    tera.add_raw_template("template", &fs::read_to_string(template_path)?)?;

    Ok(tera.render_to("template", &defaults_context, out_file)?)
}
