#[cfg(test)]
mod header {
    use string_repr::StringRepr;
    use wdg_request::header::Header;
    #[test]
    fn c7038264_ac2d_46d5_9047_02af7f3725e6() {
        let value = Header::Host("localhost").string_repr();
        assert_eq!(value, "Host: localhost");
    }
}
