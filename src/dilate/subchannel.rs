
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
    state: Option<State>,
}

impl SubChannel {
    pub fn new() -> SubChannel {
        SubChannel {
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
                RemoteData(data) => panic!(),
                RemoteClose => panic!(),
                LocalData(data) => panic!(),
                RemoteClose => panic!(),
            },
            Closing => match event {
                RemoteData(data) => panic!(),
                RemoteClose => panic!(),
                LocalData(data) => panic!(),
                RemoteClose => panic!(),
            },
            Closed => match event {
                RemoteData(data) => panic!(),
                RemoteClose => panic!(),
                LocalData(data) => panic!(),
                RemoteClose => panic!(),
            },
        });
        actions
    }
}
