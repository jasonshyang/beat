use beat::{cli::Cli, run};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse_args();
    run::run_player(cli.path)?;
    Ok(())
}
