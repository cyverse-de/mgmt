use clap::Parser;
use mgmt::app;
use mgmt::cli::deploy::Cli;
use which::which;

fn main() {
    let cli = Cli::parse();

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

    let state = app::App::from(cli);

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
