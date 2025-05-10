use fpas::gen_pass::{self, Mode};

#[test]
fn test_normal_mode() {
    let input = String::from("test");
    let result = gen_pass::normal_mode(input);
    // Normal mode should replace certain characters
    assert!(result.contains("@") || result.contains("#") || result.contains("&"));
    assert!(!result.contains("1") && !result.contains("e") && !result.contains("2"));
}

#[test]
fn test_byte_mode() {
    let input = String::from("test");
    let result = gen_pass::byte_mode(input);
    // Byte mode should contain base64 characters and replacements
    assert!(result.contains("@") || result.contains("#") || result.contains("&"));
    assert!(!result.contains("1") && !result.contains("e") && !result.contains("2"));
}

#[test]
fn test_chain_mode() {
    let input = gen_pass::normal_mode(String::from("test1234"));
    let result = gen_pass::chain_mode(gen_pass::normal_mode, input);
    // Chain mode should preserve the first part and append transformed last 4 chars
    assert_eq!(result.len(), 268);
}

#[test]
fn test_process_normal_mode() {
    let input = String::from("test");
    let result = gen_pass::process(input, Mode::Normal, 1, false);
    assert!(!result.is_empty());
}

#[test]
fn test_process_byte_mode() {
    let input = String::from("test");
    let result = gen_pass::process(input, Mode::Byte, 1, false);
    assert!(!result.is_empty());
}

#[test]
fn test_process_with_chain() {
    let input = String::from("test");
    let result = gen_pass::process(input, Mode::Normal, 1, true);
    assert!(!result.is_empty());
}

#[test]
fn test_process_with_multiple_iterations() {
    let input = String::from("test");
    let result = gen_pass::process(input, Mode::Normal, 3, false);
    assert!(!result.is_empty());
}

#[test]
fn test_process_empty_input() {
    let input = String::from("");
    let result = gen_pass::process(input, Mode::Normal, 1, false);
    assert!(!result.is_empty());
}

#[test]
fn test_process_special_characters() {
    let input = String::from("!@#$%^&*()");
    let result = gen_pass::process(input, Mode::Normal, 1, false);
    assert!(!result.is_empty());
}

#[test]
fn test_process_different_modes() {
    let input = String::from("test");
    let normal_result = gen_pass::process(input.clone(), Mode::Normal, 1, false);
    let byte_result = gen_pass::process(input, Mode::Byte, 1, false);
    assert_ne!(normal_result, byte_result);
}

#[test]
fn test_process_chain_vs_no_chain() {
    let input = String::from("test");
    let chain_result = gen_pass::process(input.clone(), Mode::Normal, 1, true);
    let no_chain_result = gen_pass::process(input, Mode::Normal, 1, false);
    assert_eq!(chain_result, no_chain_result);
}
