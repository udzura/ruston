extern crate ruston;

fn main() {
    let ret = ruston::json::parse("true");
    dbg!(ret);

    let ret = ruston::json::parse("123");
    dbg!(ret);

    let ret = ruston::json::parse("\"Hola\"");
    dbg!(ret);

    let ret = ruston::json::parse("[1, 2, 3, false]");
    dbg!(ret);
}
