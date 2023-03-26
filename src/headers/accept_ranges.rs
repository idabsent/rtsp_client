use std::{
    fmt,
    string::ToString,
};

use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::{RequestMethod},
};

pub enum AcceptRange {
    Npt,
    Smpte,
    Clock,
}

impl fmt::Display for AcceptRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let range = match self {
            Self::Npt => "npt",
            Self::Smpte => "smpte",
            Self::Clock => "clock",
        };

        write!(f, "{range}")
    }
}

pub struct AcceptRanges {
    ranges: Vec<AcceptRange>,
}

impl AcceptRanges {
    fn new() -> AcceptRanges {
        AcceptRanges {
            ranges: Vec::with_capacity(3),
        }
    }

    fn add_range(&mut self, range: AcceptRange) {
        self.ranges.push(range)
    }
}

impl Header for AcceptRanges {
    fn header() -> String {
        String::from("Accept-Ranges")
    }

    fn allow_in_methods() -> &'static [RequestMethod] {
        &[RequestMethod::Play, RequestMethod::GetParameter]
    }

    fn header_position() -> HeaderPosition {
        HeaderPosition::General
    }

    fn value(&self) -> String {
        let mut content = String::new();

        let mut peekable = self.ranges.iter().peekable();
        while let Some(range) = peekable.next() {
            content.push_str(&range.to_string());
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
    fn test_add_one_range() {
        let mut ar_header = AcceptRanges::new();
        ar_header.add_range(AcceptRange::Smpte);
        assert_eq!(&ar_header.value(), "smpte");
    }

    #[test]
    fn test_add_two_range() {
        let mut ar_header = AcceptRanges::new();
        ar_header.add_range(AcceptRange::Smpte);
        ar_header.add_range(AcceptRange::Npt);

        assert_eq!(&ar_header.value(), "smpte,npt");
    }
}