use super::{
    tools::OrdHeadersBox,
    interface::{
        Request,
        RequestMethod,
        HeadersBox
    },
};

pub struct SetParameter {
    headers_box: OrdHeadersBox,
}

impl SetParameter {
    pub fn new(headers_box: OrdHeadersBox) -> SetParameter {
        SetParameter {
            headers_box,
        }
    }
}

impl Request for SetParameter {
    type HeaderItem = <OrdHeadersBox as HeadersBox>::HeaderItem;

    fn method(&self) -> RequestMethod {
        RequestMethod::SetParameter
    }

    fn get_header(&self, header_helper: fn() -> String) -> Option<&Self::HeaderItem> {
        self.headers_box.get_header(header_helper)
    }
}