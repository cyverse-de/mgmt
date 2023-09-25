use clap::Command;
use mgmt::app;
use mgmt::cli::{configs, container_images, release, site};
use which::which;

fn main() {
    let commands = Command::new("mgmt")
        .version("0.1.0")
        .about("Discovery Environment deployment management tool")
        .args_conflicts_with_subcommands(true)
        .subcommand(configs::cli())
        .subcommand(container_images::cli())
        .subcommand(release::cli())
        .subcommand(site::cli());
    let cli = commands.get_matches();

    let skaffold_path = match which("skaffold") {
        Ok(path_buf) => path_buf,
        Err(e) => {
            println!("error finding skaffold: {}", e);
            return;
        }
    };

    let git_path = match which("git") {
        Ok(path_buf) => path_buf,
        Err(e) => {
            println!("error finding git: {}", e);
            return;
        }
    };

    let kubectl_path = match which("kubectl") {
        Ok(path_buf) => path_buf,
        Err(e) => {
            println!("error finding kubectl: {}", e);
            return;
        }
    };

    println!("skaffold path: {}", skaffold_path.display());
    println!("git path: {}", git_path.display());
    println!("kubectl path: {}", kubectl_path.display());

    let state = app::App::from(&cli).unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(1);
    });

    match state.process() {
        Ok(status) => {
            if !status {
                println!("failed to deploy");
                return;
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
