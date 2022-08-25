#[cfg(test)]
pub mod test {
    use crate::json;
    use std::error::Error;

    #[test]
    fn test_literal() -> Result<(), Box<dyn Error>> {
        let ret = json::parse(b"true");
        assert!(match ret {
            json::Value::True => true,
            _ => false,
        });

        let ret = json::parse(b"123");
        assert!(match ret {
            json::Value::Int(i) => {
                assert_eq!(123, i);
                true
            }
            _ => false,
        });

        let ret = json::parse(b"0");
        assert!(match ret {
            json::Value::Int(i) => {
                assert_eq!(0, i);
                true
            }
            _ => false,
        });

        let ret = json::parse(b"\"Hola\"");
        assert!(match ret {
            json::Value::Str(s) => {
                assert_eq!("Hola", s);
                true
            }
            _ => false,
        });

        let ret = json::parse(b"[1, 2, 3, false]");
        assert!(match ret {
            json::Value::Array(vec) => {
                assert_eq!(4, vec.len());
                true
            }
            _ => false,
        });

        let ret = json::parse(b"{\"foo\": true, \"bar\": 123}");
        assert!(match ret {
            json::Value::Object(_, _) => true,
            _ => false,
        });
        Ok(())
    }
}
