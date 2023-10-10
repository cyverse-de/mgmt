use clap::{arg, ArgAction, Command};
use std::path::PathBuf;

pub fn cli() -> Command {
    Command::new("release")
        .about("Creates and posts a release to a git repository")
        .args_conflicts_with_subcommands(true)
        .subcommand_required(true)
        .arg(
            arg!(-d --"database-url" <DATABASE>)
                .help("The URL of the MySQL database to connect to.")
                .default_value("mysql://root@127.0.0.1:3306/de_releases")
                .value_parser(clap::value_parser!(String)),
        )
        .subcommand(
            Command::new("create").args([
                arg!(-s --"skip" <SKIP> "A service to skip for the release")
                    .required(false)
                    .action(ArgAction::Append)
                    .value_parser(clap::value_parser!(String)),
                arg!(-e --env <ENV> "The environment to release")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
                arg!(-r --"repo-name" [REPO_NAME] "The repository to release to")
                    .required(false)
                    .default_value("de-releases")
                    .value_parser(clap::value_parser!(String)),
                arg!(-u --"repo-url" [REPO_URL] "The releases Git repository URL")
                    .required(false)
                    .default_value("https://github.com/cyverse-de/de-releases")
                    .value_parser(clap::value_parser!(String)),
                arg!(-b --branch [BRANCH] "The branch of the releases repo to use")
                    .required(false)
                    .default_value("main")
                    .value_parser(clap::value_parser!(String)),
                arg!(-f --"no-fail" "Do not fail if a service tarball cannot be processed")
                    .required(false)
                    .action(ArgAction::SetTrue)
                    .value_parser(clap::value_parser!(bool)),
                arg!(-n --"no-clone" "Do not clone the repository")
                    .required(false)
                    .action(ArgAction::SetTrue)
                    .value_parser(clap::value_parser!(bool)),
                arg!(--"no-push" "Do not push the changes to the repository")
                    .required(false)
                    .action(ArgAction::SetTrue)
                    .value_parser(clap::value_parser!(bool)),
                arg!(--"no-commit" "Do not commit the changes to the repository")
                    .required(false)
                    .action(ArgAction::SetTrue)
                    .value_parser(clap::value_parser!(bool)),
                arg!(--"no-tag" "Do not tag the release")
                    .required(false)
                    .action(ArgAction::SetTrue)
                    .value_parser(clap::value_parser!(bool)),
                arg!(-i --"increment-field" [INCREMENT_FIELD] "The field to increment for the release")
                    .required(false)
                    .default_value("patch")
                    .value_parser(clap::builder::PossibleValuesParser::new(["major", "minor", "patch"])),
            ]),
        )
        .subcommand(
            Command::new("deploy")
                .args([
                    arg!(-s --skip <SKIP> "A service to skip for the deployment")
                        .required(false)
                        .action(ArgAction::Append)
                        .value_parser(clap::value_parser!(String)),
                    arg!(-p --"pre-deploy" <PRE_DEPLOY> "A service to deploy before the rest")
                        .required(false)
                        .action(ArgAction::Append)
                        .value_parser(clap::value_parser!(String)),
                    arg!(-e --env <ENV> "The environment to deploy")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                    arg!(-r --"repo-name" [REPO_NAME] "The repository to deploy from")
                        .required(false)
                        .default_value("de-releases")
                        .value_parser(clap::value_parser!(PathBuf)),
                    arg!(-c --"configs" [CONFIGS] "The directory the config files will be written to")
                        .required(false)
                        .default_value("configs")
                        .value_parser(clap::value_parser!(PathBuf)),
                    arg!(-b --branch [BRANCH] "The branch of the releases repo to use")
                        .required(false)
                        .default_value("main")
                        .value_parser(clap::value_parser!(String)),
                    arg!(-D --"no-deploy" "Do not deploy the services")
                        .required(false)
                        .action(ArgAction::SetTrue)
                        .value_parser(clap::value_parser!(bool)),
                    arg!(-L --"no-load-configs" "Do not load the configs")
                        .required(false)
                        .action(ArgAction::SetTrue)
                        .value_parser(clap::value_parser!(bool)),
                    arg!(-S --"no-load-secrets" "Do not load the secrets")
                        .required(false)
                        .action(ArgAction::SetTrue)
                        .value_parser(clap::value_parser!(bool)),
                    arg!(-R --"no-render-configs" "Do not render the configs")
                        .required(false)
                        .action(ArgAction::SetTrue)
                        .value_parser(clap::value_parser!(bool)),
                ])
        )
}
