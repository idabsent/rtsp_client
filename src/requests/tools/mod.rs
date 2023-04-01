use super::{
    interface::{
        Builder,
        RequestMethod,
        BuilderError,
        Request,
        HeadersBox,
    },
    describe::Describe,
    get_parameter::GetParameter,
    options::Options,
    pause::Pause,
    play::Play,
    play_notify::PlayNotify,
    redirect::Redirect,
    set_parameter::SetParameter,
    setup::Setup,
    teardown::Teardown,
};
use crate::headers::interface::{Header, HeaderPosition};

pub struct OrdHeadersBox {
    general: Vec<Box<dyn Header>>,
    request_response: Vec<Box<dyn Header>>,
    message_body: Vec<Box<dyn Header>>,
}

impl OrdHeadersBox {
    pub fn new() -> OrdHeadersBox {
        OrdHeadersBox {
            general: vec![],
            request_response: vec![],
            message_body: vec![],
        }
    }
}

impl HeadersBox for OrdHeadersBox {
    type HeaderItem = Box<dyn Header>;
    
    fn add_header(&mut self, header: Self::HeaderItem) -> &mut Self {
        match header.header_position() {
            HeaderPosition::General => self.general.push(header),
            HeaderPosition::RequestResponse => self.request_response.push(header),
            HeaderPosition::MessageBody => self.message_body.push(header),
        }
        
        self
    }
    
    fn get_header(&self, header_helper: fn() -> String) -> Option<&Self::HeaderItem> {
        [&self.general, &self.request_response, &self.message_body]
            .into_iter()
            .flatten()
            .find(|header| header.header() == header_helper())
    }
}

impl IntoIterator for OrdHeadersBox {
    type Item = Box<dyn Header>;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    
    fn into_iter(self) -> Self::IntoIter {
        [self.general, self.request_response, self.message_body]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .into_iter()
    }
}

pub struct RequestBuilder {
    headers_box: OrdHeadersBox,
    request: Option<RequestMethod>,
}

impl Builder for RequestBuilder {
    type HeaderItem = <OrdHeadersBox as HeadersBox>::HeaderItem;
    
    fn for_request(&mut self, request: RequestMethod) -> &mut Self {
        self.request = Some(request);
        
        self
    }
    
    fn add_header(&mut self, header: Self::HeaderItem) -> &mut Self {
        if let Some(ref req) = self.request {
            if !header.allow_in_methods().contains(req) {
                eprintln!("[WARNING] Header {} is not allowed in current request method {req}. Allowed {:?}",
                    header.header(),
                    header.allow_in_methods()
                );
                
                return self;
            }
        }
        
        self.headers_box.add_header(header);
        
        self
    }

    fn build(self) -> Result<Box<dyn Request<HeaderItem = Self::HeaderItem>>, BuilderError> {
        if let None = &self.request {
            return Err(BuilderError)
        }
        
        let request = self.request.unwrap();
        
        let request: Box<dyn Request<HeaderItem = Self::HeaderItem>> = match request {
            RequestMethod::Describe => Box::new(Describe::new(self.headers_box)),
            RequestMethod::GetParameter => Box::new(GetParameter::new(self.headers_box)),
            RequestMethod::Options => Box::new(Options::new(self.headers_box)),
            RequestMethod::Pause => Box::new(Pause::new(self.headers_box)),
            RequestMethod::Play => Box::new(Play::new(self.headers_box)),
            RequestMethod::PlayNotify => Box::new(PlayNotify::new(self.headers_box)),
            RequestMethod::Redirect => Box::new(Redirect::new(self.headers_box)),
            RequestMethod::SetParameter => Box::new(SetParameter::new(self.headers_box)),
            RequestMethod::Setup => Box::new(Setup::new(self.headers_box)),
            RequestMethod::Teardown => Box::new(Teardown::new(self.headers_box)), 
        };
        
        Ok(request)
    }
}