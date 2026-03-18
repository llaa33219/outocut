use clap::{Parser, Subcommand};
use anyhow::Result;
use notify::{Watcher, RecommendedWatcher, RecursiveMode};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;
use crate::parser;

#[derive(Parser)]
#[command(name = "outocut")]
#[command(about = "AI-friendly video editor - CLI tool for motion graphics and video editing", long_about = None)]
#[command(version = "0.1.0")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Render project to video file")]
    Render {
        #[arg(help = "Path to .outocut project file")]
        project: PathBuf,

        #[arg(short, long, help = "Output file path")]
        output: PathBuf,

        #[arg(long, help = "Use GPU acceleration")]
        gpu: bool,

        #[arg(long, help = "Encoding preset ( ultrafast, fast, medium, slow, veryslow )")]
        preset: Option<String>,

        #[arg(long, help = "CRF value (0-51, lower = better quality)")]
        crf: Option<u8>,
    },

    #[command(about = "Preview project at specific time")]
    Preview {
        #[arg(help = "Path to .outocut project file")]
        project: PathBuf,

        #[arg(long, help = "Start time in seconds")]
        time: f64,

        #[arg(long, help = "Duration in seconds")]
        duration: Option<f64>,
    },

    #[command(about = "Validate project file")]
    Validate {
        #[arg(help = "Path to .outocut project file")]
        project: PathBuf,
    },

    #[command(about = "Export JSON with optional formatting")]
    ExportJson {
        #[arg(help = "Path to .outocut project file")]
        project: PathBuf,

        #[arg(short, long, help = "Pretty print JSON")]
        pretty: bool,
    },

    #[command(about = "Watch for changes and auto-reload")]
    Watch {
        #[arg(help = "Path to .outocut project file")]
        project: PathBuf,
    },
}

pub async fn watch_project(project: &PathBuf) -> Result<()> {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = notify::Watcher::new(
        move |res| {
            let _ = tx.send(res);
        },
        notify::Config::default().with_poll_interval(Duration::from_secs(1)),
    )?;

    watcher.watch(project, RecursiveMode::NonRecursive)?;

    println!("Watching {} for changes (Ctrl+C to stop)", project.display());

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                if event.kind.is_modify() {
                    println!("\n✓ Project changed, reloading...");
                    match parser::validate_project(project) {
                        Ok(_) => println!("✓ Project is valid"),
                        Err(e) => eprintln!("✗ Validation failed: {}", e),
                    }
                }
            }
            Ok(e) => eprintln!("Watch error: {:?}", e),
            Err(e) => break,
        }
    }

    Ok(())
}
