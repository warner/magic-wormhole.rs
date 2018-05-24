struct Wordlist { }

enum Event {
    C_FinishedInput(String),
    C_GotNameplate(String),
}
use self::Event::*;

enum State {
    Idle,
    WantNameplateNoNameplates,
    WantNameplateHaveNameplates(Vec<String>), // nameplates
    WantCodeNoWordlist(String),               // nameplate
    Done,
}
use self::State::*;

struct Input {
    state: State,
}

impl Input {
    fn choose_words1(&mut self, words: String) -> Vec<Event> {
        match self.state {
            Idle => panic!("too soon"),
            WantCodeNoWordlist(ref nameplate) => {
                let code = format!("{}-{}", nameplate, words);
                self.state = Done;
                vec![C_FinishedInput(code)]
            },
            _ => panic!("already set nameplate"),
        }
    }

    fn choose_words2(&mut self, words: String) -> Vec<Event> {
        let mut newstate: Option<State> = None;
        let events = match self.state {
            Idle => panic!("too soon"),
            WantCodeNoWordlist(ref nameplate) => {
                let code = format!("{}-{}", nameplate, words);
                newstate = Some(Done);
                vec![C_FinishedInput(code)]
            },
            _ => panic!("already set nameplate"),
        };
        self.state = newstate.unwrap_or(self.state);
        events
    }

    fn choose_words3(&mut self, words: String) -> Vec<Event> {
        let mut newstate: Option<State> = None;
        let events = match self.state {
            Idle => panic!("too soon"),
            WantCodeNoWordlist(ref nameplate) => {
                let code = format!("{}-{}", nameplate, words);
                newstate = Some(Done);
                vec![C_FinishedInput(code)]
            },
            _ => panic!("already set nameplate"),
        };
        let newstate2 = newstate.unwrap_or(self.state);
        self.state = newstate2;
        events
    }

    // this one compiles
    fn choose_words4(&mut self, words: String) -> Vec<Event> {
        let mut newstate: Option<State> = None;
        let events = match self.state {
            Idle => panic!("too soon"),
            WantCodeNoWordlist(ref nameplate) => {
                let code = format!("{}-{}", nameplate, words);
                newstate = Some(Done);
                vec![C_FinishedInput(code)]
            },
            _ => panic!("already set nameplate"),
        };
        if newstate.is_some() {
            self.state = newstate.unwrap();
        }
        events
    }
}
