use std::ops::RangeInclusive;
use std::collections::HashMap;

use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::{RequestMethod},
};

pub struct ContentEncoding {
    content_encodings: HashMap<String, f32>,
}

impl ContentEncoding {
    fn new() -> ContentEncoding {
        ContentEncoding {
            content_encodings: HashMap::new(),
        }
    }

    fn add_content_encoding(&mut self, content_encoding: String, qvalue: f32) {
        static ALLOWED_RANGE: RangeInclusive<f32> = 0.0 ..= 1.0;

        if !ALLOWED_RANGE.contains(&qvalue) {
            panic!("Unexpected qvalue. qvalue must be in range {ALLOWED_RANGE:?}");
        }

        self.content_encodings.insert(content_encoding, qvalue);
    }
}

impl Header for ContentEncoding {
    fn header() -> String {
        String::from("Content-Encoding")
    }

    fn allow_in_methods() -> &'static [RequestMethod] {
        &[RequestMethod::GetParameter, RequestMethod::SetParameter,
          RequestMethod::Redirect, RequestMethod::PlayNotify,
          RequestMethod::Describe, RequestMethod::Options,
          RequestMethod::Setup, RequestMethod::Play,
          RequestMethod::Pause, RequestMethod::Teardown]
    }

    fn header_position() -> HeaderPosition {
        HeaderPosition::MessageBody
    }

    fn value(&self) -> String {
        let mut content = String::new();

        let mut peekable = self.content_encodings.iter().peekable();
        while let Some((name, qvalue)) = peekable.next() {
            content.push_str(&format!("{name}; q={qvalue:.1}"));
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
    fn test_add_one_content_encoding() {
        let mut ce_header = ContentEncoding::new();
        ce_header.add_content_encoding("application/example".to_string(), 1.0);
        assert_eq!(&ce_header.value(), "application/example; q=1.0");
    }

    #[test]
    fn test_add_two_content_encoding() {
        let mut ce_header = ContentEncoding::new();
        ce_header.add_content_encoding("application/sdp".to_string(), 1.0);
        ce_header.add_content_encoding("application/example".to_string(), 0.0);
        assert_eq!(&ce_header.value(), "application/sdp; q=1.0,application/example; q=0.0");
    }
}