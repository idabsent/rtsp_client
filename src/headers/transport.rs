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
    },
    ops::{
        RangeBounds,
        Bound,
    },
    net::{
        ToSocketAddrs,
        SocketAddr,
    },
};

pub enum Profile {
    Avp,
    Savp,
    Avpf,
    Savpf,
}

impl Display for Profile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let content = match self {
            Self::Avp => "AVP",
            Self::Savp => "SAVP",
            Self::Avpf => "AVPF",
            Self::Savpf => "SAVPF",
        };

        write!(f, "{content}")
    }
}

pub enum LowerTransport {
    TCP,
    UDP,
}

impl Display for LowerTransport {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let content = match self {
            Self::TCP => "TCP",
            Self::UDP => "UDP",
        };

        write!(f, "{content}")
    }
}

pub struct RangeOptionEnd<Idx> {
    pub start: Idx,
    pub end: Option<Idx>,
}

impl<Idx> RangeBounds<Idx> for RangeOptionEnd<Idx> {
    fn start_bound(&self) -> Bound<&Idx> {
        Bound::Included(&self.start)
    }

    fn end_bound(&self) -> Bound<&Idx> {
        match &self.end {
            Some(end) => Bound::Included(end),
            None => Bound::Unbounded,
        }
    }
}

impl<Idx> Display for RangeOptionEnd<Idx>
where
    Idx: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let start = self.start.to_string();
        let end = match &self.end {
            Some(end) => end.to_string(),
            None => String::new(),
        };

        write!(f, "{start}-{end}")
    }
}

pub enum Mode {
    Play,
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let content = match self {
            Self::Play => "play",
        };

        write!(f, "{content}")
    }
}

pub enum ContransSetup {
    Active,
    Passive,
    ActPass,
}

impl Display for ContransSetup {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let content = match self {
            Self::Active => "active",
            Self::Passive => "passive",
            Self::ActPass => "actpass",
        };

        write!(f, "{content}")
    }
}

pub enum TransferType {
    Unicast,
    Multicast,
}

impl Display for TransferType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let content = match self {
            Self::Unicast => "unicast",
            Self::Multicast => "multicast",
        };

        write!(f, "{content}")
    }
}

pub enum ContransCon {
    New,
    Existing,
}

impl Display for ContransCon {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let content = match self {
            Self::New => "new",
            Self::Existing => "existing",
        };

        write!(f, "{content}")
    }
}

pub struct Transport {
    profile: Profile,
    lower_transport: LowerTransport,
    transfer_type: TransferType,
    interleaved: Option<RangeOptionEnd<i32>>,
    ttl: Option<u32>,
    layers: Option<u32>,
    mode: Mode,
    ssrc: String,
    dest_addr: Vec<SocketAddr>,
    src_addr: Vec<SocketAddr>,
    setup: Option<ContransSetup>,
    connection: Option<ContransCon>,
    rtcp_mux: bool,
    mikey: Option<String>,
}

impl Transport {
    fn new<T>(
        profile: Profile,
        lower_transport: LowerTransport,
        transfer_type: TransferType,
        mode: Mode, ssrc: String,
        dests_addrs: Option<Vec<T>>,
        src_addrs: Option<Vec<T>>) -> Transport
    where
        T: ToSocketAddrs
    {
        let dest_addr: Vec<SocketAddr> = dests_addrs
            .unwrap_or_default()
            .iter().map(|addr| addr.to_socket_addrs().unwrap().collect::<Vec<_>>())
            .flatten().collect();

        let src_addr: Vec<SocketAddr> = src_addrs
            .unwrap_or_default()
            .iter().map(|addr| addr.to_socket_addrs().unwrap().collect::<Vec<_>>())
            .flatten().collect();

        Transport {
            profile,
            lower_transport,
            transfer_type,
            interleaved: None,
            ttl: None,
            layers: None,
            mode,
            ssrc,
            dest_addr,
            src_addr,
            setup: None,
            connection: None,
            rtcp_mux: false,
            mikey: None,
        }
    }

    fn enable_interleaved<T>(&mut self, interleaved: T)
    where
        T: RangeBounds<i32>,
    {
        let start = match interleaved.start_bound() {
            Bound::Included(start) => *start,
            Bound::Excluded(_) => panic!("Uncorrect state"),
            Bound::Unbounded => panic!("Interleaved must have start number"),
        };

        let end = match interleaved.end_bound() {
            Bound::Included(end) => Some(*end),
            Bound::Excluded(_) => None,
            Bound::Unbounded => panic!("Interleaved must have start number"),
        };

        self.interleaved = Some(
            RangeOptionEnd {
                start,
                end,
            }
        );
    }

    fn enable_ttl(&mut self, ttl: u32) {
        self.ttl = Some(ttl);
    }

    fn enable_layers(&mut self, layers_count: u32) {
        self.layers = Some(layers_count);
    }

    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    fn set_ssrc(&mut self, ssrc: String) {
        self.ssrc = ssrc;
    }

    fn set_profile(&mut self, profile: Profile) {
        self.profile = profile;
    }

    fn set_lower_transport(&mut self, lower_transport: LowerTransport) {
        self.lower_transport = lower_transport;
    }

    fn set_transfer_type(&mut self, transfer_type: TransferType) {
        self.transfer_type = transfer_type;
    }

    fn enable_dest_addrs<T>(&mut self, dest_addrs: T)
    where
        T: ToSocketAddrs,
    {
        dest_addrs.to_socket_addrs()
            .unwrap()
            .into_iter()
            .for_each(|addr| self.dest_addr.push(addr));
    }

    fn enable_src_addrs<T>(&mut self, src_addrs: T)
    where
        T: ToSocketAddrs
    {
        src_addrs.to_socket_addrs()
            .unwrap()
            .into_iter()
            .for_each(|addr| self.src_addr.push(addr));
    }

    fn enable_setup(&mut self, setup: ContransSetup) {
        self.setup = Some(setup);
    }

    fn enable_connection(&mut self, connection: ContransCon) {
        self.connection = Some(connection);
    }

    fn enable_rtcp_mux(&mut self) {
        self.rtcp_mux = true;
    }

    fn enable_mikey(&mut self, mikey: String) {
        todo!()
    }
}

fn addrs_vec_to_str(addrs: &Vec<SocketAddr>) -> String {
    let mut content = String::new();

    let mut peekable = addrs.iter().peekable();
    while let Some(addr) = peekable.next() {
        content.push_str(&format!("\"{}:{}\"", addr.ip(), addr.port()));
        if let Some(_) = peekable.peek() {
            content.push('/');
        }
    }

    content
}

impl Header for Transport {
    fn header() -> String {
        String::from("Transport")
    }

    fn allow_in_methods() -> &'static [RequestMethod] {
        &[RequestMethod::Setup]
    }

    fn header_position() -> HeaderPosition {
        HeaderPosition::General
    }

    fn value(&self) -> String {
        let mut content = format!("RTP/{}/{}", self.profile, self.lower_transport);

        content.push_str(&format!(";{}", self.transfer_type));
        if let Some(ref interleaved) = self.interleaved {
            content.push_str(&format!(";interleaved={interleaved}"));
        }
        if let Some(ttl) = self.ttl {
            content.push_str(&format!(";ttl={ttl}"));
        }
        if let Some(layers) = self.layers {
            content.push_str(&format!(";layers={layers}"));
        }
        content.push_str(&format!(";ssrc={}", self.ssrc));
        content.push_str(&format!(";mode=\"{}\"", self.mode));
        self.dest_addr.is_empty().then(||
            {
                content.push_str(&format!(";dest_addr={}", addrs_vec_to_str(&self.dest_addr)));
            }
        );
        self.src_addr.is_empty().then(||
            {
                content.push_str(&format!(";src_addr={}", addrs_vec_to_str(&self.src_addr)));
            }
        );
        if let Some(ref setup) = self.setup {
            content.push_str(&format!(";setup={}", setup));
        }
        if let Some(ref connection) = self.connection {
            content.push_str(&format!(";connection={}", connection))
        }

        content
    }
}