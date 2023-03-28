use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

use chrono::{DateTime,Utc};
use std::fmt::{Display, Formatter, Error};

pub enum RandomAccess {
    RandomAccess(f32),
    BegginingOnly,
    NoSeeking,
}

impl Display for RandomAccess {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let content = match self {
            Self::RandomAccess(num) => format!("Random-Access={num:.1}"),
            Self::BegginingOnly => String::from("Beggining-Only"),
            Self::NoSeeking => String::from("No-Seeking"),
        };

        write!(f, "{content}")
    }
}

pub enum ContentModifications {
    Immutable,
    Dynamic,
    TimeProgressing,
}

impl Display for ContentModifications {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let content = match self {
            Self::Immutable => "Immutable",
            Self::Dynamic => "Dynamic",
            Self::TimeProgressing => "Time-Progressing"
        };

        write!(f, "{content}")
    }
}

pub enum Retention {
    Unlimited,
    TimeLimited(DateTime<Utc>),
    TimeDuration(f32),
}

fn absolute_time_repr(date_time: &DateTime<Utc>) -> String {
    format!("{}", date_time.format("%Y%m%d%H%M%S%.6fZ"))
}

impl Display for Retention {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let content = match self {
            Self::Unlimited => String::from("Unlimited"),
            Self::TimeLimited(date_time) => format!("Time-Limited={}", absolute_time_repr(&date_time)),
            Self::TimeDuration(num) => format!("Time-Duration={num:.1}")
        };

        write!(f, "{content}")
    }
}

pub struct MediaProperties {
    retention: Retention,
    content_mod: ContentModifications,
    random_access: RandomAccess,
}

struct Scale();

impl MediaProperties {
    fn new(retention: Retention, content_mod: ContentModifications, random_access: RandomAccess) -> MediaProperties {
        MediaProperties {
            retention,
            content_mod,
            random_access,
        }
    }

    fn set_retention(&mut self, retention: Retention) {
        self.retention = retention;
    }

    fn set_content_modification(&mut self, content_mod: ContentModifications) {
        self.content_mod = content_mod;
    }

    fn set_random_access(&mut self, random_access: RandomAccess) {
        self.random_access = random_access;
    }

    fn set_scale(&mut self, _: Scale) {
        todo!()
    }
}

impl Header for MediaProperties {
    fn header() -> String {
        String::from("Media-Properties")
    }

    fn allow_in_methods() -> &'static [RequestMethod] {
        &[RequestMethod::Setup, RequestMethod::Play,
          RequestMethod::Pause, RequestMethod::GetParameter,
          RequestMethod::PlayNotify]
    }

    fn header_position() -> HeaderPosition {
        HeaderPosition::General
    }

    fn value(&self) -> String {
        format!("{}, {}, {}",
            self.random_access,
            self.retention,
            self.content_mod,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_absolute_time_repr() {
        use chrono::{NaiveDate, NaiveTime};
        use chrono::{NaiveDateTime};

        let date = NaiveDate::from_ymd(2000, 1, 1);
        let time = NaiveTime::from_hms(0, 0, 0);

        let date_time = NaiveDateTime::new(date, time);
        let date_time = DateTime::<Utc>::from_utc(date_time, Utc);

        assert_eq!(&absolute_time_repr(&date_time), "20000101000000.000000Z");
    }

    #[test]
    fn test_string_representation() {
        let mp_header = MediaProperties::new(
                Retention::Unlimited,
                ContentModifications::Immutable,
                RandomAccess::RandomAccess(2.5),
        );

        assert_eq!(&mp_header.value(), "Random-Access=2.5, Unlimited, Immutable")
    }
}