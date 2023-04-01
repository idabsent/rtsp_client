use crate::{
    headers::interface::{Header, HeaderPosition},
    requests::interface::RequestMethod,
};

use std::{
    string::ToString,
    fmt::{
        Error,
        Display,
        Formatter,
    }
};

use chrono::{
    DateTime,
    Utc,
};

pub enum Reason {
    ServerAdmin,
    SessionTimeout,
    InternalError,
}

impl Display for Reason {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let content = match self {
            Self::ServerAdmin => "Server-Admin",
            Self::SessionTimeout => "Session-Timeout",
            Self::InternalError => "Internal-Error",
        };

        write!(f, "{content}")
    }
}

pub fn terminate_reason_helper() -> String {
    String::from("Terminate-Reason")
}

pub struct TerminateReason {
    reason: Reason,
    time: Option<DateTime<Utc>>,
    user_msg: Option<String>,
}

impl TerminateReason {
    fn new(reason: Reason) -> TerminateReason {
        TerminateReason {
            reason,
            time: None,
            user_msg: None,
        }
    }

    fn set_reason(&mut self, reason: Reason) {
        self.reason = reason;
    }

    fn set_time(&mut self, date_time: DateTime<Utc>) {
        self.time = Some(date_time);
    }

    fn set_user_msg(&mut self, msg: String) {
        self.user_msg = Some(msg);
    }
}

fn absolute_time_repr(date_time: &DateTime<Utc>) -> String {
    format!("{}", date_time.format("%Y%m%dT%H%M%S%.6fZ"))
}

impl Header for TerminateReason {
    fn header(&self) -> String {
        terminate_reason_helper()
    }

    fn allow_in_methods(&self) -> &'static [RequestMethod] {
        &[RequestMethod::Redirect, RequestMethod::Teardown]
    }

    fn header_position(&self) -> HeaderPosition {
        HeaderPosition::RequestResponse
    }

    fn value(&self) -> String {
        let mut content = self.reason.to_string();

        if let Some(dt) = &self.time {
            content.push_str(&format!("; time={}", absolute_time_repr(dt)));
        }

        if let Some(msg) = &self.user_msg {
            content.push_str(&format!("; user-msg=\"{}\"", msg));
        }

        content
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_time() -> (DateTime<Utc>, String) {
        use chrono::{
            NaiveDate,
            NaiveTime,
            NaiveDateTime,
        };

        let date = NaiveDate::from_ymd(2000, 1, 1);
        let time = NaiveTime::from_hms(0, 0, 0);
        let dt = NaiveDateTime::new(date, time);

        let dt = DateTime::<Utc>::from_utc(dt, Utc);

        (dt, "20000101T000000.000000Z".to_string())
    }

    #[test]
    fn test_with_time() {
        let (dt, dt_repr) = get_time();
        let mut tr_header = TerminateReason::new(Reason::ServerAdmin);
        tr_header.set_time(dt);

        assert_eq!(tr_header.value(), format!("Server-Admin; time={dt_repr}"));
    }

    #[test]
    fn test_with_msg() {
        //let (dt, dt_repr) = get_time();
        let mut tr_header = TerminateReason::new(Reason::ServerAdmin);
        tr_header.set_user_msg(String::from("Example msg"));

        assert_eq!(tr_header.value(), format!("Server-Admin; user-msg=\"Example msg\""));
    }
}