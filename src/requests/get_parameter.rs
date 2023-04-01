use super::{
    tools::OrdHeadersBox,
    interface::{Request, RequestMethod, HeadersBox},
};

pub struct GetParameter {
    headers_box: OrdHeadersBox,
}

impl GetParameter {
    pub fn new(headers_box: OrdHeadersBox) -> GetParameter {
        GetParameter {
            headers_box,
        }
    }
}

impl Request for GetParameter {
    type HeaderItem = <OrdHeadersBox as HeadersBox>::HeaderItem;

    fn method(&self) -> RequestMethod {
        RequestMethod::GetParameter
    }

    fn get_header(&self, header_helper: fn() -> String) -> Option<&Self::HeaderItem> {
        self.headers_box.get_header(header_helper)
    }
}