use std::{
    fs::File,
    io::{self, Read},
};

use crate::gen_pass::{self, Mode};
use clap::{command, CommandFactory, Parser};
use clap_complete::{generate, Shell};

/// Example usage of the fpas command.
const EXAMPLE_COMMANDS: &str = r#"Examples:
  fpas hello
  fpas -f path_file
"#;

/// Defines the command-line arguments for the fpas utility.
///
/// This tool generates passwords based on input strings, files, or stdin,
/// with various modes and options for customization.
#[derive(Parser, Default)]
#[command(
    name = "fpas",
    author, // Uses authors from Cargo.toml
    version, // Uses version from Cargo.toml
    about, // Uses description from Cargo.toml
    long_about = None, // Uses `about` for long description as well
    after_help = EXAMPLE_COMMANDS // Appends examples to the help message
)]
struct Cli {
    /// Input string to use for password generation.
    /// If not provided, and --file is not used, reads from stdin.
    msg: Option<String>,

    #[clap(short = 'f', long = "file", value_name = "file")]
    /// Path to a file containing the input string for password generation.
    file: Option<String>,

    #[clap(short, long, default_value_t, value_enum)]
    /// Password generation mode (e.g., character set).
    mode: Mode,

    #[clap(long, short)]
    /// Generate shell completion scripts for the specified shell.
    completions: Option<Shell>,

    #[clap(short = 'l', long = "loop", value_name = "COUNT")]
    /// Number of iterations for the generation algorithm.
    /// If chain mode is enabled, this determines the number of segments concatenated.
    loop_count: Option<u32>,

    #[clap(long, default_value_t = false)]
    /// Enable chain mode: generates a longer password by concatenating results from multiple loops.
    chain: bool,
}

/// Helper enum to distinguish input sources for reading.
enum InputSource<'a> {
    File(File),
    Stdin(io::StdinLock<'a>),
}

/// Reads data from the given source and attempts to convert it to a UTF-8 string.
/// Exits the program with an error message if reading or UTF-8 conversion fails.
fn read_source_to_string(source: InputSource, source_name: &str) -> String {
    let mut buffer = Vec::new();
    let read_result = match source {
        InputSource::File(mut f) => f.read_to_end(&mut buffer),
        InputSource::Stdin(mut s) => s.read_to_end(&mut buffer),
    };

    if let Err(e) = read_result {
        eprintln!("Error: Failed to read from {}: {}", source_name, e);
        std::process::exit(1);
    }

    match String::from_utf8(buffer) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: Input from {} is not valid UTF-8: {}", source_name, e);
            std::process::exit(1);
        }
    }
}

/// Runs the fpas CLI application logic.
///
/// Parses command-line arguments, handles input from various sources (argument, file, stdin),
/// generates shell completions if requested, and then generates and prints the password.
pub fn run() {
    let cli = Cli::parse();

    // Handle shell completion generation if requested.
    if let Some(shell) = cli.completions {
        let mut cli_gen = Cli::command();
        generate(shell, &mut cli_gen, "fpas", &mut io::stdout());
        return;
    }

    // Determine the input string for password generation.
    // Priority: 1. Direct message argument, 2. File content, 3. Stdin.
    let input_data: String = if let Some(message_arg) = cli.msg {
        message_arg
    } else if let Some(file_path_str) = cli.file {
        match File::open(&file_path_str) {
            Ok(file) => read_source_to_string(InputSource::File(file), &file_path_str),
            Err(e) => {
                eprintln!("Error: Could not open file '{}': {}", file_path_str, e);
                std::process::exit(1);
            }
        }
    } else {
        // Read from stdin if no message or file is provided.
        let stdin = std::io::stdin();
        let handle = stdin.lock();
        read_source_to_string(InputSource::Stdin(handle), "stdin")
    };

    // Determine the loop count, defaulting to 1 if not specified.
    let iterations = cli.loop_count.unwrap_or(1);

    // Generate the password.
    let passwd = gen_pass::process(input_data, cli.mode, iterations, cli.chain);

    // Print the generated password to stdout.
    print!("{}", passwd);
}
