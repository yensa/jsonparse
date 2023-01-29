pub mod json_lexer;

use std::{fs::File, io::Read};

use crate::json_lexer::json_tokenize;


fn main() {
    /*
    JSONParser :
        parse(char&) -> $ AST $
    */
    let mut f = File::open("example.json").unwrap();

    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();

    match json_tokenize(content) {
        Ok(result) => {
            println!("{:?}", result);
        }
        Err(error) => {
            println!("There was an error {}", error);
        }
    }
}
