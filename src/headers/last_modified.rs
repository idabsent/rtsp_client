use chrono::{
    DateTime, Local,
};

use crate::{
    headers::interface::{HeaderPosition, Header},
    requests::interface::RequestMethod,
};

pub fn last_modified_helper() -> String {
    String::from("Last-Modified")
}

pub struct LastModified {
    date_time: DateTime<Local>,
}

impl LastModified {
    fn new(date_time: DateTime<Local>) -> LastModified {
        let now = Local::now();
        if date_time > now {
            panic!("date in future. Now: {now} | Date: {date_time}")
        }

        LastModified {
            date_time
        }
    }

    fn set_date_time(&mut self, date_time: DateTime<Local>) {
        self.date_time = date_time;
    }
}

impl Header for LastModified {
    fn header(&self) -> String {
        last_modified_helper()
    }

    fn allow_in_methods(&self) -> &'static [RequestMethod] {
        &[RequestMethod::Describe, RequestMethod::Setup,
          RequestMethod::GetParameter]
    }

    fn header_position(&self) -> HeaderPosition {
        HeaderPosition::RequestResponse
    }

    fn value(&self) -> String {
        format!("{}", self.date_time.format("%a, %b %Y %X GMT"))
    }
}