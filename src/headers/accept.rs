use std::ops::RangeInclusive;
use std::collections::HashMap;

use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

pub struct Accept {
    content_types: HashMap<String, f32>,
}

impl Accept {
    fn new() -> Accept {
        Accept {
            content_types: HashMap::new(),
        }
    }

    fn add_content_type<'a>(&mut self, content_type: &'a str, qvalue: f32) {
        static ALLOW_RANGE: RangeInclusive<f32> = 0.0 ..= 1.0;

        if !ALLOW_RANGE.contains(&qvalue) {
            panic!("Unexpected qvalue. qvalue must be in {ALLOW_RANGE:?}");
        }

        self.content_types.insert(content_type.to_string(), qvalue);
    }
}

impl Header for Accept {
    fn header() -> String {
        String::from("Accept")
    }

    fn allow_in_methods() -> &'static [RequestMethod] {
        &[RequestMethod::Describe,]
    }

    fn header_position() -> HeaderPosition {
        HeaderPosition::RequestResponse
    }

    fn value(&self) -> String {
        let mut content = String::new();

        let mut peekable = self.content_types.iter().peekable();
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
    fn test_one_content_type() {
        let mut a_header = Accept::new();
        a_header.add_content_type("application/sdp", 1.0);
        assert_eq!(&a_header.value(), "application/sdp; q=1.0");
    }

    #[test]
    fn test_two_content_type() {
        let mut a_header = Accept::new();
        a_header.add_content_type("application/sdp", 1.0);
        a_header.add_content_type("application/example", 0.0);
        assert_eq!(&a_header.value(), "application/sdp; q=1.0,application/example; q=0.0");
    }
}