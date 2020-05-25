#![allow(dead_code, unused_variables)]

use std::cmp::min;
use std::convert::TryInto;
use super::types::Frame;

pub struct RecordError(String);

pub struct PingID([u8; 4]);
pub struct Seqnum(u32);
pub struct SubchannelID(u32);

pub enum Record {
    KCM,
    Ping(PingID),
    Pong(PingID),
    Open(SubchannelID, Seqnum),
    Data(SubchannelID, Seqnum, Vec<u8>),
    Close(SubchannelID, Seqnum),
    Ack(Seqnum),
}

const T_KCM: u8 = b'\x00';
const T_PING: u8 = b'\x01';
const T_PONG: u8 = b'\x02';
const T_OPEN: u8 = b'\x03';
const T_DATA: u8 = b'\x04';
const T_CLOSE: u8 = b'\x05';
const T_ACK: u8 = b'\x06';

pub struct ParseError(String);
impl From<ParseError> for RecordError {
    fn from(e: ParseError) -> RecordError {
        RecordError(format!("parse error: {}", e.0))
    }
}

fn extract_4bytes(plaintext: &[u8], offset: usize) -> [u8; 4] {
    plaintext[offset..offset+4].try_into().unwrap()
}

fn extract_be4(plaintext: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes(extract_4bytes(plaintext, offset))
}

fn parse_record(plaintext: &[u8]) -> Result<Record, ParseError> {
    use Record::*;
    if plaintext.len() == 0 {
        return Err(ParseError("received empty message".to_string()));
    }
    let expected_length = match plaintext[0] {
        T_KCM => 1,
        T_PING | T_PONG => 1+4,
        T_OPEN | T_CLOSE => 1+4+4,
        T_DATA => min(1+4+4, plaintext.len()),
        T_ACK => 1+4,
        _ => return Err(ParseError(format!("received unknown message type {}",
                                           plaintext[0]))),
    };
    if plaintext.len() != expected_length {
        return Err(ParseError(format!("received leftover data: got {}, wanted {}", plaintext.len(), expected_length)));
    }

    let record = match plaintext[0] {
        T_KCM => KCM,
        T_PING => Ping(PingID(extract_4bytes(plaintext, 1))),
        T_PONG => Pong(PingID(extract_4bytes(plaintext, 1))),
        T_OPEN => Open(SubchannelID(extract_be4(plaintext, 1)),
                       Seqnum(extract_be4(plaintext, 1+4))),
        T_DATA => Data(SubchannelID(extract_be4(plaintext, 1)),
                       Seqnum(extract_be4(plaintext, 1+4)),
                       plaintext[1+4+4..].to_vec()),
        T_CLOSE => Close(SubchannelID(extract_be4(plaintext, 1)),
                         Seqnum(extract_be4(plaintext, 1+4))),
        T_ACK => Ack(Seqnum(extract_be4(plaintext, 1))),
        _ => panic!(),
    };
    Ok(record)
}

fn encode_u32(id: u32) -> Vec<u8> {
    id.to_be_bytes().to_vec()
}

fn encode_record(record: Record) -> Vec<u8> {
    use Record::*;
    match record {
        KCM => vec![vec![T_KCM]],
        Ping(id) => vec![vec![T_PING], id.0.to_vec()],
        Pong(id) => vec![vec![T_PONG], id.0.to_vec()],
        Open(scid, seqnum) => vec![vec![T_OPEN], encode_u32(scid.0), encode_u32(seqnum.0)],
        Data(scid, seqnum, data) => vec![vec![T_DATA], encode_u32(scid.0), encode_u32(seqnum.0), data],
        Close(scid, seqnum) => vec![vec![T_CLOSE], encode_u32(scid.0), encode_u32(seqnum.0)],
        Ack(seqnum) => vec![vec![T_ACK], encode_u32(seqnum.0)],
    }.concat()

}

pub enum Input {
    FrameReceived(Frame),
    SendRecord(Record),
}

pub enum Output {
    GotHandshake,
    GotRecord(Record),
    SendFrame(Frame),
}

pub type RecordResult = Result<Vec<Output>, RecordError>;

#[derive(Debug)]
pub enum State {
    WantPrologue,
    WantHandshake,
    WantMessage,
}

pub enum Role {
    Leader,
    Follower,
}

pub struct DecryptError(String);
type DecryptResult = Result<Vec<u8>, DecryptError>;
impl From<DecryptError> for RecordError {
    fn from(e: DecryptError) -> RecordError {
        RecordError(format!("decrypt error: {}", e.0))
    }
}

// fake for now
pub struct Noise {
}
impl Noise {
    pub fn write_message(&mut self) -> Vec<u8> {
        // returns handshake
        vec![]
    }
    pub fn read_message(&mut self, message: Vec<u8>) -> DecryptResult {
        Ok(vec![])
    }
    pub fn encrypt(&mut self, plaintext: Vec<u8>) -> Vec<u8> {
        vec![]
    }
    pub fn decrypt(&mut self, ciphertext: Vec<u8>) -> DecryptResult {
        Ok(vec![])
    }
}

pub struct RecordHandler {
    role: Role,
    state: State,
    noise: Noise,
}

fn nothing() -> RecordResult {
    Ok(vec![])
}

impl RecordHandler {
    pub fn new(role: Role) -> RecordHandler {
        RecordHandler { role, state: State::WantPrologue,
                        noise: Noise{} }
    }

    pub fn process(&mut self, input: Input) -> RecordResult {
        use Input::*;
        match input {
            FrameReceived(frame) => self.frame_received(frame),
            SendRecord(record) => self.send_record(record),
        }
    }

    fn send_handshake(&mut self) -> RecordResult {
        // generate the ephemeral keys
        let handshake = Frame(self.noise.write_message());
        Ok(vec![Output::SendFrame(handshake)])
    }

    fn process_handshake(&mut self, frame: Frame) -> RecordResult {
        // Noise can include unencrypted data in the handshake, but we don't
        // use it
        let _payload = self.noise.read_message(frame.0)?;
        Ok(vec![])
    }

    fn frame_received(&mut self, frame: Frame) -> RecordResult {
        use State::*;
        match self.state {
            WantPrologue => {
                self.state = WantHandshake;
                match self.role {
                    Role::Leader => self.send_handshake(),
                    Role::Follower => nothing(),
                }
            },
            WantHandshake => {
                self.state = WantMessage;
                self.process_handshake(frame)?;
                match self.role {
                    Role::Leader => nothing(),
                    Role::Follower => self.send_handshake(),
                }
            },
            WantMessage => {
                let record = parse_record(&self.noise.decrypt(frame.0)?)?;
                Ok(vec![Output::GotRecord(record)])
            },
        }
    }

    fn send_record(&mut self, record: Record) -> RecordResult {
        if let State::WantMessage = self.state {
            let frame = Frame(self.noise.encrypt(encode_record(record)));
            Ok(vec![Output::SendFrame(frame)])
        } else {
            panic!("send_record while in state {:?}", self.state);
        }
    }

}
