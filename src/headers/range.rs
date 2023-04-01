use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

use chrono::{
    NaiveTime,
    DateTime,
    Utc,
};
use std::fmt::{
    Display,
    Error,
    Formatter,
};

use std::ops::{Bound, RangeBounds};

pub struct Smpte {
    time: NaiveTime,
    frames: u8,
    subframes: u8,
}

impl Smpte {
    fn new(time: NaiveTime, frames: u8, subframes: u8) -> Smpte {
        Smpte {
            time,
            frames,
            subframes,
        }
    }
}

impl Display for Smpte {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let time_str = format!("{}", self.time.format("%H:%M:%S"));

        write!(f, "{time_str}:{}.{}", self.frames, self.subframes)
    }
}

pub enum NptType {
    Seconds(f32),
    Full(NaiveTime),
    Now,
}

impl Display for NptType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let content = match self {
            Self::Seconds(secs) => format!("{secs:.1}"),
            Self::Full(time) => format!("{}", time.format("%H:%M:%S%.1f")),
            Self::Now => String::from("now"),
        };

        write!(f, "{content}")
    }
}

pub struct Npt<T>
where
    T: RangeBounds<NptType>
{
    range: T,
}

impl<T> Display for Npt<T>
where
    T: RangeBounds<NptType>
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let start = self.range.start_bound();
        let end = self.range.end_bound();

        let start_str = match start {
            Bound::Unbounded => String::from("0.0"),
            Bound::Included(num) => format!("{num}"),
            Bound::Excluded(_) => panic!("Unexpected state")
        };

        let end_str = match end {
            Bound::Unbounded => String::new(),
            Bound::Included(num) => format!("{num}"),
            Bound::Excluded(_) => panic!("Unexpected state. End of range must be inclusive"),
        };

        write!(f, "{start_str}-{end_str}")
    }
}

fn absolute_time_repr(date_time: &DateTime<Utc>) -> String {
    format!("{}", date_time.format("%Y%m%d%H%M%S%.6fZ"))
}

pub fn range_helper() -> String {
    String::from("Range")
}

pub enum Range<T>
where
    T: RangeBounds<NptType>
{
    Smpte(Smpte),
    Npt(Npt<T>),
    Clock(DateTime<Utc>)
}

impl<T> Header for Range<T>
where
    T: RangeBounds<NptType>
{
    fn header(&self) -> String {
        range_helper()
    }

    fn allow_in_methods(&self) -> &'static [RequestMethod] {
        &[RequestMethod::Setup, RequestMethod::Play, RequestMethod::Pause,
          RequestMethod::GetParameter, RequestMethod::PlayNotify]
    }

    fn header_position(&self) -> HeaderPosition {
        HeaderPosition::RequestResponse
    }

    fn value(&self) -> String {
        match self {
            Self::Smpte(smpte) => smpte.to_string(),
            Self::Npt(npt) => npt.to_string(),
            Self::Clock(dt) => absolute_time_repr(dt),
        }
    }
}