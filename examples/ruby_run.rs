extern crate ruston;
use rb_sys as rb;

fn main() {
    //     let json = "
    // {\"Hello\": true, \"world\": [1, 2, 3]}
    // ";

    let json = "true";
    let ruby = ruston::ruby::parse_into_ruby(json.to_string()).unwrap();
    unsafe { rb::rb_p(ruby) };

    let json = "\"Rubyist?\"";
    let ruby = ruston::ruby::parse_into_ruby(json.to_string()).unwrap();
    unsafe { rb::rb_p(ruby) };

    let json = "[\"Rubyist?\", 12, null]";
    let ruby = ruston::ruby::parse_into_ruby(json.to_string()).unwrap();
    unsafe { rb::rb_p(ruby) };

    let json = "{\"Rubyist?\": true}";
    let ruby = ruston::ruby::parse_into_ruby(json.to_string()).unwrap();
    unsafe { rb::rb_p(ruby) };
}
