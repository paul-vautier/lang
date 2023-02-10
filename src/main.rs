use std::fs;

mod lang;

fn main() {
    match lang::tokens::tokenize(
        fs::read_to_string("./test.txt")
            .expect("could not find specified file")
            .as_str(),
    ) {
        Ok(tokens) => println!("{:?}", tokens),
        Err(error) => println!("Error : {}", error),
    }
}
