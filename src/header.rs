use string_repr::StringRepr;

pub struct Header(String, String);

impl Header {
    pub fn new(name: String, value: String) -> Header {
        Header(name, value)
    }
}

impl StringRepr for Header {
    fn string_repr(&self) -> String {
        format!("{}: {}", self.0, self.1)
    }
}
