pub struct Header(String, String);

impl Header {
    pub fn new(name: String, value: String) -> Header {
        Header(name, value)
    }
}
