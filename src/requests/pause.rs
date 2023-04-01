use super::{
    tools::OrdHeadersBox,
    interface::{Request, RequestMethod, HeadersBox},
};

pub struct Pause {
    headers_box: OrdHeadersBox,
}

impl Pause {
    pub fn new(headers_box: OrdHeadersBox) -> Pause {
        Pause {
            headers_box,
        }
    }
}

impl Request for Pause {
    type HeaderItem = <OrdHeadersBox as HeadersBox>::HeaderItem;

    fn method(&self) -> RequestMethod {
        RequestMethod::Pause
    }

    fn get_header(&self, header_helper: fn() -> String) -> Option<&Self::HeaderItem>
    {
        self.headers_box.get_header(header_helper)
    }
}