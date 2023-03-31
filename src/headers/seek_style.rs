use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

pub enum SeekStyle {
    Rap,
    CoRap,
    FirstPrior,
    Next,
}

impl Header for SeekStyle {
    fn header(&self) -> String {
        String::from("Seek-Style")
    }

    fn allow_in_methods(&self) -> &'static [RequestMethod] {
        &[RequestMethod::Play]
    }

    fn header_position(&self) -> HeaderPosition {
        HeaderPosition::General
    }

    fn value(&self) -> String {
        let content = match self {
            Self::Rap => "Rap",
            Self::CoRap => "CoRAP",
            Self::FirstPrior => "First-Prior",
            Self::Next => "Next",
        };

        String::from(content)
    }
}