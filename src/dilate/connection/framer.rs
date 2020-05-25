#![allow(dead_code)]

use std::str;
use std::convert::TryInto;

pub struct FramerError(String);

enum Expected {
    Found,
    Waiting,
}
type ExpectedResult = Result<Expected, FramerError>;

// Look for a specific string at the beginning of the buffer
fn get_expected(buffer: &mut Vec<u8>, name: &str, expected: &[u8])
                -> ExpectedResult {
    use Expected::*;
    let lb = buffer.len();
    let le = expected.len();
    if buffer.starts_with(&expected) {
        // if the buffer starts with the expected string, consume it and
        // return Found
        *buffer = buffer.split_off(le);
        return Ok(Found);
    }
    if !expected.starts_with(&buffer) {
        // we're not on track: the data we've received so far does not
        // match the expected value, so this can't possibly be right.
        // Don't complain until we see the expected length, or a newline,
        // so we can capture the weird input in the log for debugging.
        if buffer.contains(&b'\n') || lb >= le {
            let (left, _) = buffer.split_at(le);
            let msg = format!("bad {}: {}", name, str::from_utf8(left).unwrap());
            return Err(FramerError(msg.to_string()));
        }
        // keep collecting more bad data to improve the error message
        return Ok(Waiting);
    }
    // good so far, just waiting for the rest
    return Ok(Waiting);
}

pub struct Frame(Vec<u8>);

fn frame_data(frame: Frame) -> Vec<u8> {
    let len = frame.0.len();
    let mut data = Vec::with_capacity(4 + len);
    data.extend_from_slice(&len.to_be_bytes());
    data.extend_from_slice(&frame.0);
    data
}


pub enum Input {
    // IO
    Connected,
    Data(Vec<u8>),
    Disconnected,
    // Output
    SendFrame(Frame),
}

pub enum Output {
    // IO
    Send(Vec<u8>),
    // Events
    GotPrologue,
    GotFrame(Frame),
    Disconnected,
}

pub struct RelayHandshakes {
    outbound: Vec<u8>,
    expected_inbound: Vec<u8>, // always b'ok\n'
}

enum State {
    NotConnected,
    WantRelay,
    WantPrologue,
    WantFrame,
    Disconnected,
}

pub type FramerResult = Result<Vec<Output>, FramerError>;


pub struct Framer {
    state: State,
    buffer: Vec<u8>,
    relay_handshakes: Option<RelayHandshakes>,
    inbound_prologue: Vec<u8>,
    outbound_prologue: Vec<u8>,
}

impl Framer {
    pub fn new(relay_handshakes: Option<RelayHandshakes>,
               inbound_prologue: Vec<u8>,
               outbound_prologue: Vec<u8>) -> Framer {
        use State::*;
        let state = match relay_handshakes {
            Some(_) => WantRelay,
            None => WantPrologue,
        };
        Framer {
            state,
            buffer: vec![],
            relay_handshakes,
            inbound_prologue,
            outbound_prologue,
        }
    }

    pub fn process(&mut self, input: Input) -> FramerResult {
        use Input::*;
        match input {
            Connected => self.connected(),
            Data(data) => self.data_received(data),
            Disconnected => self.disconnected(),
            SendFrame(frame) => self.send_frame(frame),
        }
    }

    fn connected(&mut self) -> FramerResult {
        use State::*;
        match self.state {
            NotConnected => {
                match self.relay_handshakes {
                    Some(ref handshakes) => {
                        self.state = WantRelay;
                        Ok(vec![Output::Send(handshakes.outbound.clone())])
                    },
                    None => {
                        self.state = WantPrologue;
                        Ok(vec![])
                    },
                }
            },
            _ => Err(FramerError("already connected".to_string())),
        }
    }

    fn data_received(&mut self, data: Vec<u8>) -> FramerResult {
        if let State::NotConnected = self.state {
            panic!("data before connected");
        }
        self.buffer.extend_from_slice(&data);
        let mut output = vec![];
        loop {
            let mut out = self.parse_next()?;
            if out.is_empty() {
                return Ok(output);
            } else {
                output.append(&mut out);
            }
        }
    }

    fn parse_next(&mut self) -> FramerResult {
        use State::*;
        use Expected::*;
        use Output::*;
        match self.state {
            NotConnected => panic!("data before connected"),
            WantRelay => {
                let expected = &self.relay_handshakes.as_ref().unwrap().expected_inbound;
                match get_expected(&mut self.buffer, "relay_ok", &expected)? {
                    Found => {
                        self.state = WantPrologue;
                        Ok(vec![Send(self.outbound_prologue.clone())])
                    }
                    Waiting => Ok(vec![]),
                }
            },
            WantPrologue => {
                match get_expected(&mut self.buffer, "prologue", &self.inbound_prologue)? {
                    Found => Ok(vec![GotPrologue]),
                    Waiting => Ok(vec![]),
                }
            },
            WantFrame => {
                match self.parse_frame() {
                    Some(frame) => Ok(vec![GotFrame(frame)]),
                    None => Ok(vec![]),
                }
            },
            State::Disconnected => panic!("data after disconnect"),
        }
    }

    fn parse_frame(&mut self) -> Option<Frame> {
        if self.buffer.len() < 4 {
            return None;
        }
        let lp = self.buffer[0..4].try_into().unwrap();
        let frame_length = u32::from_be_bytes(lp);
        if self.buffer.len() < (4 + frame_length) as usize {
            return None;
        }
        // discard the length prefix, then peel off the frame
        let mut frame = self.buffer.split_off(4);
        self.buffer = frame.split_off(frame_length.try_into().unwrap());
        Some(Frame(frame))
    }

    fn disconnected(&mut self) -> FramerResult {
        use State::*;
        match self.state {
            NotConnected => panic!("disconnect before connect"),
            Disconnected => panic!("double disconnect"),
            _ => {
                self.state = Disconnected;
                Ok(vec![Output::Disconnected])
            },
        }
    }

    fn send_frame(&mut self, frame: Frame) -> FramerResult {
        use State::*;
        match self.state {
            NotConnected => panic!("send_frame before connect"),
            WantRelay => panic!("send_frame while still WantRelay"),
            WantPrologue => panic!("send_frame while still WantPrologue"),
            WantFrame => {
                let data = frame_data(frame);
                Ok(vec![Output::Send(data)])
            },
            Disconnected => panic!("send_frame after disconnect"),
        }
    }

}
