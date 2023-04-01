use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

pub enum OctetCount {
    KB(u32),
    MB(u32),
    GB(u32),
}

impl OctetCount {
    fn bit_count(&self) -> u32 {
        match self {
            Self::KB(kb) => kb * 1024,
            Self::MB(mb) => mb * 1024 * 1024,
            Self::GB(gb) => gb * 1024 * 1024 * 1024,
        }
    }
}

pub fn blocksize_helper() -> String {
    String::from("Blocksize")
}

pub struct Blocksize {
    size: OctetCount,
}

impl Blocksize {
    fn new(size: OctetCount) -> Blocksize {
        Blocksize {
            size,
        }
    }

    fn set_size(&mut self, size: OctetCount) {
        self.size = size
    }
}

impl Header for Blocksize {
    fn header(&self) -> String {
        blocksize_helper()
    }

    fn allow_in_methods(&self) -> &'static [RequestMethod] {
        &[RequestMethod::Setup, RequestMethod::Describe,
          RequestMethod::SetParameter, RequestMethod::Play]
    }

    fn header_position(&self) -> HeaderPosition {
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
        let b_header = Blocksize::new(OctetCount::KB(1));
        assert_eq!(&b_header.value(), "1024");
    }

    #[test]
    fn test_mb() {
        let b_header = Blocksize::new(OctetCount::MB(1));
        assert_eq!(&b_header.value(), "1048576");
    }

    #[test]
    fn test_gb() {
        let b_header = Blocksize::new(OctetCount::GB(1));
        assert_eq!(&b_header.value(), "1073741824");
    }
}