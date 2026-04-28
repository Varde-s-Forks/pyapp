use anyhow::Result;
use clap::builder::styling::Styles;
use clap::{Parser, Subcommand};

pub const CLAP_STYLING: Styles = Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);

#[derive(Parser, Debug)]
#[command(
    bin_name = env!("PYAPP_PROJECT_NAME"),
    version,
    disable_help_subcommand = true,
    styles = CLAP_STYLING
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(name = env!("PYAPP_SELF_COMMAND"))]
    SelfCmd(super::self_cmd::cli::Cli),
}

impl Cli {
    pub fn exec(self) -> Result<()> {
        match self.command {
            Commands::SelfCmd(cli) => cli.exec(),
        }
    }
}
