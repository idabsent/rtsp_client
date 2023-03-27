use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

pub struct CSeq {
    number: u32,
}

impl CSeq {
    fn new(begin: u32) -> CSeq {
        CSeq {
            number: begin,
        }
    }

    fn set_number(&mut self, number: u32) {
        self.number = number;
    }
}

impl Iterator for CSeq {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(number) = self.number.checked_add(1) {
            self.number = number;
            return Some(self.number)
        }

        None
    }
}

impl Header for CSeq {
    fn header() -> String {
        String::from("CSeq")
    }

    fn allow_in_methods() -> &'static [RequestMethod] {
        &[RequestMethod::Describe, RequestMethod::Play,
          RequestMethod::Setup, RequestMethod::Options,
          RequestMethod::Pause, RequestMethod::Teardown,
          RequestMethod::GetParameter, RequestMethod::SetParameter,
          RequestMethod::Redirect, RequestMethod::PlayNotify]
    }

    fn header_position() -> HeaderPosition {
        HeaderPosition::General
    }

    fn value(&self) -> String {
        self.number.to_string()
    }
}

#[cfg(test)]
mod cfg {
    use super::*;

    #[test]
    fn test_next_some() {
        use std::u32;

        let mut cs_header = CSeq::new(0);
        assert_eq!(cs_header.next(), Some(1));
    }

    #[test]
    fn test_next_none() {
        use std::u32;

        let mut cs_header = CSeq::new(u32::MAX);
        assert_eq!(cs_header.next(), None);
    }
}