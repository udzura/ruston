extern crate ruston;

fn main() {
    //     let json = "
    // {\"Hello\": true, \"world\": [1, 2, 3]}
    // ";

    let json = "true";
    ruston::ruby::parse_into_ruby(json.to_string()).unwrap();

    let json = "\"Rubyist?\"";
    ruston::ruby::parse_into_ruby(json.to_string()).unwrap();

    let json = "[\"Rubyist?\", 12, null]";
    ruston::ruby::parse_into_ruby(json.to_string()).unwrap();
}
