use nom::*;
use nom::types::CompleteStr;

// TODO: figure how to make parse_i32 and parse_i64 generic...
// TODO: parse should no panic, but should use nom's return_error

/// Parse an i32 from the supplied `CompleteStr`.
named!(pub parse_i32<CompleteStr, i32>,
    alt!(
        do_parse!(
            take_while!(char::is_whitespace)    >>
            tag!("-")                           >>
            int: take_while!(char::is_numeric)  >>
            
            (-1 * int.parse::<i32>().expect("parse<i32>"))
        ) |
        do_parse!(
            take_while!(char::is_whitespace)    >>
            int: take_while!(char::is_numeric)  >>

            (int.parse::<i32>().expect("parse<i32>"))
        )
    )
);

/// Parse an i64 from the supplied `CompleteStr`.
named!(pub parse_i64<CompleteStr, i64>,
    alt!(
        do_parse!(
            take_while!(char::is_whitespace)    >>
            tag!("-")                           >>
            int: take_while!(char::is_numeric)  >>
            
            (-1 * int.parse::<i64>().expect("parse<i64>"))
        ) |
        do_parse!(
            take_while!(char::is_whitespace)    >>
            int: take_while!(char::is_numeric)  >>

            (int.parse::<i64>().expect("parse<i64>"))
        )
    )
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_i32() {
        assert_eq!(parse_i32(CompleteStr("12")).unwrap().1, 12);
        assert_eq!(parse_i32(CompleteStr("    12, ")).unwrap().1, 12);
        assert_eq!(parse_i32(CompleteStr("-5")).unwrap().1, -5);
        assert_eq!(parse_i32(CompleteStr("    -12, ")).unwrap().1, -12);
    }

    #[test]
    #[should_panic]
    fn test_parse_i32_panic1() {
        assert!(parse_i32(CompleteStr(" #12")).is_err());
    }

    #[test]
    #[should_panic]
    fn test_parse_i32_panic2() {
            assert!(parse_i32(CompleteStr("  12")).is_err());
    }
}
