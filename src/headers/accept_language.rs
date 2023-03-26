use std::ops::RangeInclusive;
use std::collections::HashMap;

use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::{RequestMethod},
};

pub struct AcceptLanguage {
    content_languages: HashMap<String, f32>,
}

impl AcceptLanguage {
    fn new() -> AcceptLanguage {
        AcceptLanguage {
            content_languages: HashMap::new(),
        }
    }

    fn add_content_language<'a>(&mut self, content_lang: &'a str, qvalue: f32) {
        static ALLOW_RANGE: RangeInclusive<f32> = 0.0 ..= 1.0;

        if !ALLOW_RANGE.contains(&qvalue) {
            panic!("Unexpected qvalue. qvalue must be in {ALLOW_RANGE:?}");
        }

        self.content_languages.insert(content_lang.to_string(), qvalue);
    }
}

impl Header for AcceptLanguage {
    fn header() -> String where Self: Sized {
        String::from("Accept-Language")
    }

    fn allow_in_methods() -> &'static [RequestMethod] where Self: Sized {
        &[RequestMethod::Describe, RequestMethod::GetParameter,
          RequestMethod::SetParameter, RequestMethod::Redirect]
    }

    fn header_position() -> HeaderPosition where Self: Sized {
        HeaderPosition::RequestResponse
    }

    fn value(&self) -> String {
        let mut content = String::new();

        let mut peekable = self.content_languages.iter().peekable();
        while let Some((name, qvalue)) = peekable.next() {
            content.push_str(&format!("{name}; q={qvalue:.1}"));
            if let Some(_) = peekable.peek() {
                content.push(',')
            }
        }

        content
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one_content_language() {
        let mut al_header = AcceptLanguage::new();
        al_header.add_content_language("ru", 1.0);
        assert_eq!(&al_header.value(), "ru; q=1.0");
    }

    #[test]
    fn test_two_content_language() {
        let mut al_header = AcceptLanguage::new();
        al_header.add_content_language("ru", 1.0);
        al_header.add_content_language("en", 0.0);
        assert_eq!(&al_header.value(), "ru; q=1.0,en; q=0.0");
    }
}