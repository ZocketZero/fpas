pub fn print_passwd(passwd: &String, len:Option<u8>   ) {
    if let Some(len) = len {
        let passwd: Vec<char> = passwd.chars().collect();
        let passwd_len = passwd.len();
        for i in 0..len as usize {
            if i >= passwd_len {
                break;
            }
            print!("{}", passwd[i]);
        }
    } else {
        // Print the generated password to stdout.
        print!("{}", passwd);
    }
}
