use std::fmt;

use crate::headers::interface::{
    Header,
};

#[derive(PartialEq, Eq, Debug)]
pub enum RequestMethod {
    Describe,
    GetParameter,
    SetParameter,
    Options,
    Pause,
    Play,
    PlayNotify,
    Redirect,
    Setup,
    Teardown,
}

impl fmt::Display for RequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let name = match self {
            Self::Describe => "DESCRIBE",
            Self::GetParameter => "GET_PARAMETER",
            Self::SetParameter => "SET_PARAMETER",
            Self::Options => "OPTIONS",
            Self::Pause => "PAUSE",
            Self::Play => "PLAY",
            Self::PlayNotify => "PLAY_NOTIFY",
            Self::Redirect => "REDIRECT",
            Self::Setup => "SETUP",
            Self::Teardown => "TEARDOWN",
        };

        write!(f, "{}", name)
    }
}

pub struct BuilderError;

pub trait HeadersBox {
    type HeaderItem;

    fn add_header(&mut self, header: Self::HeaderItem) -> &mut Self;
    fn get_header(&self, header_helper: fn() -> String) -> Option<&Self::HeaderItem>;
}

pub trait Builder {
    type HeaderItem;

    fn for_request(&mut self, request: RequestMethod) -> &mut Self;
    fn add_header(&mut self, header: Self::HeaderItem) -> &mut Self;
    fn build(self) -> Result<Box<dyn Request<HeaderItem = Self::HeaderItem>>, BuilderError>;
}

pub trait Request {
    type HeaderItem;

    fn method(&self) -> RequestMethod;
    fn get_header(&self, header_helper: fn() -> String) -> Option<&Self::HeaderItem>;
}