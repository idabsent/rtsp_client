use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::{RequestMethod},
};

use chrono::{
    DateTime, Local,
};

struct Date {
    date: DateTime<Local>,
}

impl Date {
    fn new(date: DateTime<Local>) -> Date {
        let now = Local::now();

        if now < date {
            panic!("date in future. Now: {now} | Date: {date}")
        }

        Date {
            date
        }
    }

    fn set_date(&mut self, date: DateTime<Local>) {
        self.date = date;
    }
}

impl Header for Date {
    fn header(&self) -> String {
        String::from("Date")
    }

    fn allow_in_methods(&self) -> &'static [RequestMethod] {
        &[RequestMethod::Describe, RequestMethod::Setup,
          RequestMethod::Options, RequestMethod::Play,
          RequestMethod::Pause, RequestMethod::Teardown,
          RequestMethod::GetParameter, RequestMethod::SetParameter,
          RequestMethod::PlayNotify, RequestMethod::Redirect]
    }

    fn header_position(&self) -> HeaderPosition {
        HeaderPosition::General
    }

    fn value(&self) -> String {
        format!("{}", self.date.format("%a, %b %e %Y %X GMT"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_date_rep() {
        use chrono::{NaiveDate, NaiveTime};
        use chrono::{NaiveDateTime, FixedOffset};

        let date = NaiveDate::from_ymd(2022, 4, 19);
        let time = NaiveTime::from_hms(0, 0, 0);
        let dt = NaiveDateTime::new(date, time);
        let offset = FixedOffset::east_opt(5 * 3600).unwrap();
        let date_time = DateTime::<Local>::from_local(dt, offset);

        let date = Date::new(date_time);
        assert_eq!(&date.value(), "Tue, Apr 19 2022 00:00:00 GMT");
    }
}