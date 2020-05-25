/*
pub enum ConnectionEvent {
    ConnectionMade,
    DataReceived(Vec<u8>),
}

enum Role {
    Leader,
    Follower,
}

struct Connection {
    state: Option<State>,
    role: Role,
    description: String,
    //connector: ???,
    //noise: ??,
    outboundPrologue: Vec<u8>,
    inboundPrologue: Vec<u8>,
    useRelay: bool,
    relayHandshake: Vec<u8>,
}

enum State {
    Unselected,
    Selecting,
    Selected,
}

impl Connection {
    pub fn process(&mut self, e: ConnectionEvent) -> Events {
        use self::State::*;
        let old_state = self.state.take().unwrap();
        let mut actions = Events::new();
        match e {
            ConnectionMade => self.
    }
}
*/

pub mod types;
pub mod framer;
pub mod record;
