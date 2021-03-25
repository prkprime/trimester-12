use fancy_regex::Regex;
use lazy_static::lazy_static;

fn is_valid_passwd(passwd: &str) -> bool {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,20}$")
                .unwrap();
    }
    RE.is_match(passwd).unwrap()
}

fn main() {
    let passwords = vec!["Abcd@1234", "abcd@1234", "ABCD1234", "123afd", "fsf68erfuwerfie7ew8ew9887we", "ABCD^1234"];
    for passwd in passwords.iter() {
        match is_valid_passwd(passwd) {
            true => println!("{} : Valid", passwd),
            false => println!("{} : Invalid", passwd)
        }
    }
}
