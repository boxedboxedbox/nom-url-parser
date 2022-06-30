mod link;
use link::*;

fn main() {}

#[test]
fn parser_test_1() {
    assert_eq!(
        parse_link("https://cat.ru:80/vodka-pics"),
        Ok(Link {
            protocol: Protocol::HTTPS,
            domain: String::from("cat.ru"),
            port: Some(80),
            path: vec!["vodka-pics"]
        })
    )
}

#[test]
fn parser_test_2() {
    assert_eq!(
        parse_link("http://doggo.org/"),
        Ok(Link {
            protocol: Protocol::HTTP,
            domain: String::from("doggo.org"),
            port: None,
            path: vec![]
        })
    )
}

// panics as it should
#[test]
fn parser_test_3() {
    assert_ne!(
        parse_link("ftp://foo.bar:8000"),
        Ok(Link {
            protocol: Protocol::HTTP,
            domain: String::from("foo.bar"),
            port: Some(8000),
            path: vec![]
        })
    )
}

// panics as it should
#[test]
fn parser_test_4() {
    assert_ne!(
        parse_link("http::/a.b:1/"),
        Ok(Link {
            protocol: Protocol::HTTP,
            domain: String::from("a.b"),
            port: Some(1),
            path: vec![]
        })
    )
}
