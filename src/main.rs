use clap::{Parser, Subcommand};
use colored::Colorize;
use std::path::PathBuf;
use std::process::Command as Cmd;

#[derive(Parser)]
#[command(about = "Interactive cluster shell built with Rust")]
#[command(version = "0.1.0", author = "Binsk lee")]
#[command(disable_help_flag = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path about hosts file
    #[arg(short = 'h', long = "hosts", value_name = "FILE")]
    hosts: Option<PathBuf>,

    /// Output directory
    #[arg(short = 'o', long = "outdir", value_name = "DIR")]
    outdir: Option<PathBuf>,

    /// User to login
    #[arg(short = 'l', long = "user", value_name = "USER")]
    user: Option<String>,

    /// Options to pass to the ssh
    #[arg(short = 'O', long = "option", value_name = "OPTION")]
    options: Option<Vec<String>>,

    /// Timeout in seconds
    #[arg(short = 't', long = "timeout", value_name = "SECONDS")]
    timeout: Option<u64>,
}

#[derive(Subcommand)]
enum Commands {
    /// Specific command to run
    Exec { cmd: Option<Vec<String>> },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Exec { cmd } => {
            let output = execute_command(cmd.unwrap_or_else(|| {
                eprintln!("No command string provided");
                std::process::exit(1);
            }))
            .unwrap_or_else(|err| {
                eprint!(
                    "Error is occured during execute command! {}",
                    err.to_string().red()
                );
                std::process::exit(1);
            });
            print!("{}", output.green());
        }
        _ => unimplemented!(),
    };
}

fn execute_command(command: Vec<String>) -> Result<String, std::io::Error> {
    let result = Cmd::new("sh").arg("-c").arg(command.join(" ")).output()?;

    if !result.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{}", String::from_utf8_lossy(&result.stderr)),
        ));
    }

    let output_string = String::from_utf8_lossy(&result.stdout);
    Ok(output_string.into_owned())
}
