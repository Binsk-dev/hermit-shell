use clap::{CommandFactory, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(about = "Interactive cluster shell built with Rust")]
#[command(version = "0.1.0", author = "Binsk lee")]
#[command(disable_help_flag = true)]
struct Cli {
    /// Command to execute
    command: Option<String>,

    /// Print help information
    #[arg(long = "help", exclusive = true)]
    help: bool,

    /// Path to the hosts file("host:[:port] [user])
    #[arg(short = 'h', long = "hosts", value_name = "FILE")]
    hosts: Option<PathBuf>,

    /// Output directory
    #[arg(short = 'o', long = "outdir", value_name = "DIR")]
    outdir: Option<PathBuf>,

    /// User to login
    #[arg(short = 'u', long = "user", value_name = "USER")]
    user: Option<String>,

    /// Options to pass to the ssh
    #[arg(short = 'O', long = "option", value_name = "OPTION")]
    options: Option<Vec<String>>,

    /// Timeout in seconds
    #[arg(short = 't', long = "timeout", value_name = "SECONDS")]
    timeout: Option<u64>,
}

#[derive(Subcommand)]
enum Command {
    /// Help subcommand
    Help {
        /// The command to get help for
        command: Option<String>,
    },
}

enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
}

fn colorize(text: &str, color: Color) -> String {
    match color {
        Color::Red => format!("\x1b[31m{}\x1b[0m", text),
        Color::Green => format!("\x1b[32m{}\x1b[0m", text),
        Color::Yellow => format!("\x1b[33m{}\x1b[0m", text),
        Color::Blue => format!("\x1b[34m{}\x1b[0m", text),
        Color::Magenta => format!("\x1b[35m{}\x1b[0m", text),
    }
}

fn display(output: &str, command: &str) {
    println!("-------::Output::--------");
    println!("Running cmd: {}", colorize(command, Color::Green));
    println!("{}", colorize(output, Color::Yellow));
    println!("--------------------------");
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    if cli.help {
        let _ = Cli::command().print_help();
        return Ok(());
    }

    match cli.command {
        Some(command) => {
            let result = std::process::Command::new("sh")
                .arg("-c")
                .arg(command.as_str())
                .output()?;
            let output_str = String::from_utf8_lossy(&result.stdout);

            if !result.status.success() {
                eprintln!("Error: {:?}", result.status.code());
            } else {
                display(output_str.as_ref(), command.as_str());
            }
        }
        None => {
            println!("No command provided");
        }
    }

    Ok(())
}
