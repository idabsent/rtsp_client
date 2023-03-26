use std::ops::RangeInclusive;
use std::collections::HashMap;

use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

pub struct AcceptEncoding {
    content_encoders: HashMap<String, f32>,
}

impl AcceptEncoding {
    fn new() -> AcceptEncoding {
        AcceptEncoding {
            content_encoders: HashMap::new(),
        }
    }

    fn add_content_encoder<'a>(&mut self, content_encoder: &'a str, qvalue: f32) {
        static ALLOW_RANGE: RangeInclusive<f32> = 0.0 ..= 1.0;

        if !ALLOW_RANGE.contains(&qvalue) {
            panic!("Unexpected qvalue. qvalue must be in {ALLOW_RANGE:?}");
        }

        self.content_encoders.insert(content_encoder.to_string(), qvalue);
    }
}

impl Header for AcceptEncoding {
    fn header() -> String {
        String::from("Accept-Encoding")
    }

    fn allow_in_methods() -> &'static [RequestMethod] {
        &[RequestMethod::Describe,]
    }

    fn header_position() -> HeaderPosition {
        HeaderPosition::RequestResponse
    }

    fn value(&self) -> String {
        let mut content = String::new();

        let mut peekable = self.content_encoders.iter().peekable();
        while let Some((encoder, qvalue)) = peekable.next() {
            content.push_str(&format!("{encoder}; q={qvalue:.1}"));
            if let Some(_) = peekable.peek() {
                content.push(',');
            }
        }

        content
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one_content_encoder() {
        let mut ae_header = AcceptEncoding::new();
        ae_header.add_content_encoder("application/gzip", 1.0);
        assert_eq!(&ae_header.value(), "application/gzip; q=1.0");
    }

    #[test]
    fn test_two_content_encoder() {
        let mut ae_header = AcceptEncoding::new();
        ae_header.add_content_encoder("application/gzip", 1.0);
        ae_header.add_content_encoder("application/example", 0.0);
        assert_eq!(&ae_header.value(), "application/gzip; q=1.0,application/example; q=0.0");
    }

    #[test]
    #[should_panic]
    fn test_qvalue_move_range() {
        let mut ae_header = AcceptEncoding::new();
        ae_header.add_content_encoder("", -1.0);
    }
}