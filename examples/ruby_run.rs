extern crate ruston;
use rb_sys as rb;

fn main() {
    //     let json = "
    // {\"Hello\": true, \"world\": [1, 2, 3]}
    // ";

    let json = b"true";
    let ruby = ruston::ruby::parse_into_ruby(json).unwrap();
    unsafe { rb::rb_p(ruby) };

    let json = b"\"Rubyist?\"";
    let ruby = ruston::ruby::parse_into_ruby(json).unwrap();
    unsafe { rb::rb_p(ruby) };

    let json = b"[\"Rubyist?\", 12, null]";
    let ruby = ruston::ruby::parse_into_ruby(json).unwrap();
    unsafe { rb::rb_p(ruby) };

    let json = b"{\"Rubyist?\": true}";
    let ruby = ruston::ruby::parse_into_ruby(json).unwrap();
    unsafe { rb::rb_p(ruby) };
}
