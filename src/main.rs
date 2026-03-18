use anyhow::Result;
use clap::Parser;
use tracing::info;

mod cli;
mod parser;
mod models;
mod composition;
mod animation;
mod render;

use cli::Commands;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let args = cli::Args::parse();

    match args.command {
        Commands::Render {
            project,
            output,
            gpu,
            preset,
            crf,
        } => {
            info!("Rendering project: {} -> {}", project.display(), output.display());
            render::render_project(&project, &output, gpu, preset, crf).await?;
        }
        Commands::Preview {
            project,
            time,
            duration,
        } => {
            info!("Previewing project: {} at {}s", project.display(), time);
            render::preview_project(&project, time, duration).await?;
        }
        Commands::Validate { project } => {
            info!("Validating project: {}", project.display());
            parser::validate_project(&project)?;
            println!("✓ Project is valid");
        }
        Commands::ExportJson {
            project,
            pretty,
        } => {
            info!("Exporting JSON: {}", project.display());
            let json = parser::export_json(&project, pretty)?;
            println!("{}", json);
        }
        Commands::Watch { project } => {
            info!("Watching project: {}", project.display());
            cli::watch_project(&project).await?;
        }
    }

    Ok(())
}
