use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

use std::ops::RangeInclusive;

pub struct Speed {
    range: RangeInclusive<f32>,
}

impl Speed {
    fn new(range: RangeInclusive<f32>) -> Speed {
        Speed {
            range,
        }
    }

    fn set_session(&mut self, range: RangeInclusive<f32>) {
        self.range = range
    }
}

impl Header for Speed {
    fn header() -> String {
        String::from("Session")
    }

    fn allow_in_methods() -> &'static [RequestMethod] {
        &[RequestMethod::Play]
    }

    fn header_position() -> HeaderPosition {
        HeaderPosition::General
    }

    fn value(&self) -> String {
        format!("{:.1}-{:.1}", self.range.start(), self.range.end())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string_repr() {
        let s_header = Speed::new(1.01..=2.01);
        assert_eq!(&s_header.value(), "1.0-2.0")
    }
}