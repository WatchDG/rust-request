use string_repr::StringRepr;

pub enum Header<'a> {
    Allow(&'a str),
    Host(&'a str),
    UserAgent(&'a str),
    Referer(&'a str),
    Custom(&'a str, &'a str),
}

impl<'a> Header<'a> {
    pub fn new(name: &'a str, value: &'a str) -> Header<'a> {
        Header::Custom(name, value)
    }
}

impl StringRepr for Header<'_> {
    fn string_repr(&self) -> String {
        match self {
            Header::Allow(hv) => format!("Allow: {}", *hv),
            Header::Host(hv) => format!("Host: {}", *hv),
            Header::UserAgent(hv) => format!("User-Agent: {}", *hv),
            Header::Referer(hv) => format!("Referer: {}", *hv),
            Header::Custom(hn, hv) => format!("{}: {}", *hn, *hv),
        }
    }
}
