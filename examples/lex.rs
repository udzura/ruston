extern crate ruston;

fn main() {
    let json = "
{\"Hello\": true, \"world\": [1, 2, 3]}
";
    let ret = ruston::json::lexer::Lex::run(json.to_string())
        .unwrap()
        .tokens;
    dbg!(ret);
}
