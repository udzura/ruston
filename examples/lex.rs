extern crate ruston;

fn main() {
    let json = b"
{\"Hello\": true, \"world\": [1, 2, 3]}
";
    let ret = ruston::json::lexer::Lex::run(json).unwrap().tokens;
    dbg!(ret);
}
