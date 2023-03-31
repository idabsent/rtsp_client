use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

use chrono::{
    DateTime,
    Utc,
};

pub enum RetryAfter {
    DateTime(DateTime<Utc>),
    Seconds(u32),
}

impl Header for RetryAfter {
    fn header(&self) -> String {
        String::from("Require")
    }

    fn allow_in_methods(&self) -> &'static [RequestMethod] {
        &[RequestMethod::Describe, RequestMethod::Setup,
          RequestMethod::Play, RequestMethod::Options,
          RequestMethod::Pause, RequestMethod::Teardown,
          RequestMethod::GetParameter, RequestMethod::SetParameter,
          RequestMethod::Redirect, RequestMethod::PlayNotify]
    }

    fn header_position(&self) -> HeaderPosition {
        HeaderPosition::RequestResponse
    }

    fn value(&self) -> String {
        match self {
            Self::DateTime(dt) => format!("{}", dt.format("%a, %e %b %Y %X GMT")),
            Self::Seconds(secs) => secs.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string_representation() {
        use chrono::{
            NaiveDateTime,
            NaiveDate,
            NaiveTime,
        };

        let date = NaiveDate::from_ymd(1999, 12, 31);
        let time = NaiveTime::from_hms(23, 59, 59);
        let dt = NaiveDateTime::new(date, time);
        let dt = DateTime::<Utc>::from_utc(dt, Utc);
        let ra_header = RetryAfter::DateTime(dt);
        assert_eq!(&ra_header.value(), "Fri, 31 Dec 1999 23:59:59 GMT");
    }
}