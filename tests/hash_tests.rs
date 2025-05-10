use fpas::hash;

#[test]
fn test_md5() {
    let input = String::from("test");
    let result = hash::md5(&input);
    assert_eq!(result, "098f6bcd4621d373cade4e832627b4f6");
}

#[test]
fn test_sha1() {
    let input = String::from("test");
    let result = hash::sha1(&input);
    assert_eq!(result, "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3");
}

#[test]
fn test_sha256() {
    let input = String::from("test");
    let result = hash::sha256(&input);
    assert_eq!(
        result,
        "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"
    );
}

#[test]
fn test_to_base64() {
    let input = String::from("48656c6c6f"); // "Hello" in hex
    let result = hash::to_base64(input);
    assert_eq!(result, "SGVsbG8=");
}

#[test]
fn test_hash_empty_string() {
    let input = String::from("");
    assert_eq!(hash::md5(&input), "d41d8cd98f00b204e9800998ecf8427e");
    assert_eq!(
        hash::sha1(&input),
        "da39a3ee5e6b4b0d3255bfef95601890afd80709"
    );
    assert_eq!(
        hash::sha256(&input),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

#[test]
fn test_hash_special_characters() {
    let input = String::from("!@#$%^&*()");
    let md5_result = hash::md5(&input);
    let sha1_result = hash::sha1(&input);
    let sha256_result = hash::sha256(&input);

    assert!(!md5_result.is_empty());
    assert!(!sha1_result.is_empty());
    assert!(!sha256_result.is_empty());
}
