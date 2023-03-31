use std::fmt;

use crate::headers::interface::{
    Header,
};

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

pub trait HeadersBox {
    type HeaderItem;

    fn add_header(&mut self, header: Self::HeaderItem);
}

pub trait Request {
    fn method(&self) -> RequestMethod;
    fn add_header(&mut self, header: Box<dyn Header>);
    fn get_header<T>(&self, field: &T) -> Box<dyn Header> where T: Fn() -> String;
}