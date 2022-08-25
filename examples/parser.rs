extern crate ruston;

fn main() {
    let ret = ruston::json::parse(b"true");
    dbg!(ret);

    let ret = ruston::json::parse(b"123");
    dbg!(ret);

    let ret = ruston::json::parse(b"\"Hola\"");
    dbg!(ret);

    let ret = ruston::json::parse(b"[1, 2, 3, false]");
    dbg!(ret);

    let ret = ruston::json::parse(b"{\"foo\": true, \"bar\": 123}");
    dbg!(ret);

    let nested = b"
{
  \"id\": 1,
  \"user\": {
    \"name\": \"Akubi\",
    \"age\": 3,
  }
}
";
    let ret = ruston::json::parse(nested);
    dbg!(ret);
}
