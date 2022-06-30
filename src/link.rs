use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till, take_until1},
    *,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Protocol {
    HTTP,
    HTTPS,
}

impl From<&str> for Protocol {
    fn from(i: &str) -> Self {
        match i.to_lowercase().as_str() {
            "http://" => Protocol::HTTP,
            "https://" => Protocol::HTTPS,
            _ => unimplemented!("no other protocol supported"),
        }
    }
}

/// Parsed link
#[derive(Debug, PartialEq, Eq)]
pub struct Link<'a> {
    pub protocol: Protocol,
    pub domain: String,
    pub port: Option<u16>,
    pub path: Vec<&'a str>,
}

fn parse_protocol<'a>(input: &'a str) -> IResult<&'a str, Protocol, &'a str> {
    let protocol: IResult<&'a str, &'a str, ()> = alt((tag("http://"), tag("https://")))(input);

    Ok(match protocol {
        Ok((i, o)) => (i, o.into()),
        Err(_) => return Err(nom::Err::Error("Protocol not supported!")),
    })
}

fn parse_domain<'a>(input: &'a str) -> IResult<&'a str, String, &'a str> {
    let mut ret: (&'a str, String) = ("", String::new()); // input, result

    match take_until1::<_, _, ()>(".")(input) {
        Ok((i, o)) => {
            ret.0 = i;
            ret.1 = o.to_string()
        }
        Err(_) => return Err(nom::Err::Error("Bad domain name.")),
    }

    match take_till::<_, _, ()>(|c| c == '/' || c == ':' || c == '\n' || c == '\t' || c == ' ')(
        ret.0,
    ) {
        Ok((i, o)) => {
            ret.0 = i;
            ret.1.push_str(o);

            Ok((ret.0, ret.1))
        }
        Err(_) => Err(nom::Err::Error("Bad domain TLD.")),
    }
}

fn parse_port<'a>(input: &'a str) -> IResult<&'a str, Option<u16>, &'a str> {
    if input.len() < 2 || &input[0..1] != ":" {
        return Ok((input, None));
    }

    let i_dunno: IResult<&'a str, &'a str, ()> = take(1usize)(input);

    let i0 = i_dunno.unwrap().0;

    match take_till::<_, _, ()>(|c| c == '/' || c == '\n' || c == '\t' || c == ' ')(i0) {
        Ok((i, o)) => Ok((i, Some(o.parse::<u16>().unwrap()))),
        Err(_) => Err(nom::Err::Error("Bad port.")),
    }
}

fn parse_path<'a>(input: &'a str) -> IResult<&'a str, Vec<&'a str>, &'a str> {
    if input.len() < 2 || &input[0..1] != "/" {
        return Ok((input, vec![]));
    }

    let i_dunno: IResult<&'a str, &'a str, ()> = take(1usize)(input);

    let (mut i0, mut path) = (i_dunno.unwrap().0, Vec::new());

    match take_till::<_, _, ()>(|c| c == '/' || c == '\n' || c == '\t' || c == ' ')(i0) {
        Ok((i, o)) => {
            i0 = i;
            path.push(o)
        }
        Err(_) => return Err(nom::Err::Error("Bad path.")),
    }

    Ok((i0, path))
}

/// Parse an URL and turn it into usable struct.
///  
/// # Panics
///
/// if you suck at typing
/// if you use subdomains
pub fn parse_link(link: &str) -> Result<Link, String> {
    let (input, protocol) = parse_protocol(link).unwrap();
    println!("i0: {}", input);
    let (input1, domain) = parse_domain(input).unwrap();
    println!("i1: {}", input1);
    let (input2, port) = parse_port(input1).unwrap();
    println!("i2: {}", input2);
    let (_, path) = parse_path(input2).unwrap();

    Ok(Link {
        protocol,
        domain,
        port,
        path,
    })
}
