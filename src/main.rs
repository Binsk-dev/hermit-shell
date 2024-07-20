use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Interactive cluster shell built with Rust")]
#[command(version = "0.1.0", author = "Binsk lee")]
#[command(next_line_help = true)]
struct Cli {
    /// Command to execute
    command: Option<String>,
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
