use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

pub enum BitCount {
    Kb(u32),
    Mb(u32),
    Gb(u32),
}

impl BitCount {
    fn bit_count(&self) -> u32 {
        match self {
            Self::Kb(kb) => *kb,
            Self::Mb(mb) => mb * 1000,
            Self::Gb(gb) => gb * 1000 * 1000,
        }
    }
}

pub struct Bandwith {
    size: BitCount,
}

impl Bandwith {
    fn new(size: BitCount) -> Bandwith {
        Bandwith {
            size,
        }
    }

    fn set_size(&mut self, size: BitCount) {
        self.size = size
    }
}

impl Header for Bandwith {
    fn header() -> String {
        String::from("Bandwith")
    }

    fn allow_in_methods() -> &'static [RequestMethod] {
        &[RequestMethod::Setup, RequestMethod::Describe,
          RequestMethod::Options, RequestMethod::Play]
    }

    fn header_position() -> HeaderPosition {
        HeaderPosition::RequestResponse
    }

    fn value(&self) -> String {
        self.size.bit_count().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_kb() {
        let b_header = Bandwith::new(BitCount::Kb(1000));
        assert_eq!(&b_header.value(), "1000");
    }

    #[test]
    fn test_mb() {
        let b_header = Bandwith::new(BitCount::Mb(1000));
        assert_eq!(&b_header.value(), "1000000");
    }

    #[test]
    fn test_gb() {
        let b_header = Bandwith::new(BitCount::Gb(1000));
        assert_eq!(&b_header.value(), "1000000000");
    }
}