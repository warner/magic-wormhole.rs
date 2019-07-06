
#[derive(Debug, PartialEq)]
enum State {
    Open,
    Closing,
    Closed,
}

enum SubChannelEvent {
    RemoteData(Vec<u8>),
    RemoteClose,
    LocalData(Vec<u8>),
    LocalClose,
}

pub struct SubChannel {
    scid: u32,
    state: Option<State>,
}

impl SubChannel {
    pub fn new(scid: u32) -> SubChannel {
        SubChannel {
            scid,
            state: Some(State::Open),
        }
    }

    pub fn process(&mut self, event: SubChannelEvent) -> Events {
        let old_state = self.state.take().unwrap();
        let mut actions = Events::new();
        use State::*;
        use SubChannelEvent::*;
        self.state = Some(match old_state {
            Open => match event {
                LocalData(data) => { send_data(data); Open }
                RemoteData(data) => { signal_dataReceived(data); Open }
                LocalClose => { send_close(); Closing }
                RemoteClose => { send_close(); signal_connectionLost(); Closed }
            },
            Closing => match event {
                LocalData(data) => panic!("write not allowed on closed subchannel"),
                RemoteData(data) => { signal_dataReceived(data); Closing }
                LocalClose => panic!("loseConnection not allowed on closed subchannel"),
                RemoteClose => { signal_connectionLost(); Closed }
            },
            Closed => panic!("we should be deleted by now, no messages"),
        });
        actions
    }
}
