mod app;
mod cli;
mod configs;
mod git;

use crate::cli::Cli;
use clap::Parser;
use which::which;

fn main() {
    let cli = Cli::parse();

    let skaffold_path = match which("skaffold") {
        Ok(path_buf) => path_buf,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let git_path = match which("git") {
        Ok(path_buf) => path_buf,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    println!("skaffold path is {}", skaffold_path.display());
    println!("git path is {}", git_path.display());

    let state = app::App::from(cli);
    state.print_fields();

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
