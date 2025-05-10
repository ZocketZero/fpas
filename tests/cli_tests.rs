use clap::Parser;
use fpas::cli;
use fpas::gen_pass::Mode;

#[test]
fn test_read_source_to_string_file() {
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_str().unwrap().to_string();
    std::fs::write(&file_path, "test content").unwrap();

    let file = std::fs::File::open(&file_path).unwrap();
    let result = cli::read_source_to_string(cli::InputSource::File(file), &file_path);
    assert_eq!(result, "test content");
}

#[test]
fn test_cli_parse() {
    let args = vec!["fpas", "test"];
    let cli = cli::Cli::parse_from(args);
    assert_eq!(cli.msg, Some("test".to_string()));
    assert_eq!(cli.file, None);
    assert_eq!(cli.mode, Mode::N);
    assert_eq!(cli.loop_count, None);
    assert_eq!(cli.chain, false);
}

#[test]
fn test_cli_parse_with_file() {
    let args = vec!["fpas", "-f", "test.txt"];
    let cli = cli::Cli::parse_from(args);
    assert_eq!(cli.msg, None);
    assert_eq!(cli.file, Some("test.txt".to_string()));
}

#[test]
fn test_cli_parse_with_mode() {
    let args = vec!["fpas", "test", "--mode", "byte"];
    let cli = cli::Cli::parse_from(args);
    assert_eq!(cli.mode, Mode::Byte);
}

#[test]
fn test_cli_parse_with_loop_count() {
    let args = vec!["fpas", "test", "-l", "5"];
    let cli = cli::Cli::parse_from(args);
    assert_eq!(cli.loop_count, Some(5));
}

#[test]
fn test_cli_parse_with_chain() {
    let args = vec!["fpas", "test", "--chain"];
    let cli = cli::Cli::parse_from(args);
    assert_eq!(cli.chain, true);
}

#[test]
fn test_cli_parse_with_completions() {
    let args = vec!["fpas", "--completions", "bash"];
    let cli = cli::Cli::parse_from(args);
    assert!(cli.completions.is_some());
}

#[test]
fn test_cli_parse_with_multiple_options() {
    let args = vec![
        "fpas", "test", "-f", "test.txt", "-l", "3", "--chain", "--mode", "byte",
    ];
    let cli = cli::Cli::parse_from(args);
    assert_eq!(cli.msg, Some("test".to_string()));
    assert_eq!(cli.file, Some("test.txt".to_string()));
    assert_eq!(cli.loop_count, Some(3));
    assert_eq!(cli.chain, true);
    assert_eq!(cli.mode, Mode::Byte);
}
