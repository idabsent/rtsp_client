use std::ops::RangeInclusive;
use std::collections::HashMap;

use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::{RequestMethod},
};

pub struct ContentLanguage {
    content_languages: HashMap<String, f32>,
}

impl ContentLanguage {
    fn new() -> ContentLanguage {
        ContentLanguage {
            content_languages: HashMap::new(),
        }
    }

    fn add_content_language(&mut self, content_language: String, qvalue: f32) {
        static ALLOW_RANGE: RangeInclusive<f32> = 0.0 ..= 1.0;

        if !ALLOW_RANGE.contains(&qvalue) {
            panic!("Unexpected qvalue. qvalue must be in {ALLOW_RANGE:?}");
        }

        self.content_languages.insert(content_language, qvalue);
    }
}

impl Header for ContentLanguage {
    fn header() -> String {
        String::from("Content-Language")
    }

    fn allow_in_methods() -> &'static [RequestMethod] {
        &[RequestMethod::Describe, RequestMethod::Options,
          RequestMethod::Setup, RequestMethod::Play,
          RequestMethod::Pause, RequestMethod::Teardown,
          RequestMethod::GetParameter, RequestMethod::SetParameter,
          RequestMethod::PlayNotify, RequestMethod::Redirect]
    }

    fn header_position() -> HeaderPosition {
        HeaderPosition::MessageBody
    }

    fn value(&self) -> String {
        let mut content = String::new();

        let mut peekable = self.content_languages.iter().peekable();
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
    fn test_add_one_language() {
        let mut cl_header = ContentLanguage::new();
        cl_header.add_content_language("ru".to_string(), 1.0);
        assert_eq!(&cl_header.value(), "ru; q=1.0");
    }

    #[test]
    fn test_add_two_langauges() {
        let mut cl_header = ContentLanguage::new();
        cl_header.add_content_language("ru".to_string(), 1.0);
        cl_header.add_content_language("en".to_string(), 0.0);
        assert_eq!(&cl_header.value(), "ru; q=1.0,en; q=0.0");
    }
}