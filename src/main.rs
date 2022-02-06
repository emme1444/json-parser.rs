use std::process;

use parser::parser::Parser;

fn main() {
    let mut parser = Parser::new_without_comments();
    // let mut parser = Parser::new_with_comments();

    let s = "\
    {
        \"key1\": \"value\",
        \"key2\": null,
        \"key3\": false,
        \"key4\": [
            \"value1\",
            \"value2\",
            \"value3\"
        ]
    }
    ";
    // let s = "null ";
    // let s = "[]";
    // let s = "[null]";
    // let s = "{\"key\": null /* hello */}";

    // let result = parser.parse(&" \n \n  // hello null\nnull\"$\"".to_string());
    // let result = parser.parse(&"\"$\"".to_string());
    let result = parser.parse(&s.to_string());
    // let result = parser.parse(&" \n a".to_string());
    if let Err(err) = result {
        println!("Could not parse: {}", err);
        process::exit(1);
    }

    println!("\n\nResult: \n{:#?}", result.unwrap());
}
