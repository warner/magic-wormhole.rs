use std::sync::Arc;
use machine::{machine, transitions};
use super::events::{Code, Events, Nameplate, Wordlist};

// we process these
use super::events::CodeEvent;
// we emit these
use super::events::AllocatorEvent::Allocate as A_Allocate;
/*
use super::events::BossEvent::GotCode as B_GotCode;
use super::events::InputEvent::Start as I_Start;
use super::events::KeyEvent::GotCode as K_GotCode;
use super::events::NameplateEvent::SetNameplate as N_SetNameplate;
*/

machine!(
    enum CodeStateMachine {
        Idle,
        InputtingNameplate,
        InputtingWords,
        Allocating,
        Known,
    }
);

#[derive(Clone, Debug, PartialEq)]
pub struct AllocateCode<'a> {
    wordlist: Arc<Wordlist>,
    evs: &'a mut Events,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InputCode<'a> {
    evs: &'a mut Events,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetCode<'a> {
    code: Code,
    evs: &'a mut Events,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Allocated<'a> {
    nameplate: Nameplate,
    code: Code,
    evs: &'a mut Events,
}
#[derive(Clone, Debug, PartialEq)]
pub struct GotNameplate<'a> {
    nameplate: Nameplate,
    evs: &'a mut Events,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FinishedInput<'a> {
    code: Code,
    evs: &'a mut Events,
}

transitions!(CodeStateMachine,
             [
                 (Idle, AllocateCode) => Allocating,
                 (Idle, InputCode) => InputtingNameplate,
                 (Idle, SetCode) => Known,
                 (InputtingNameplate, GotNameplate) => InputtingWords,
                 (InputtingWords, FinishedInput) => Known,
                 (Allocating, Allocated) => Known
             ]);

impl Idle {
    pub fn on_allocate_code(self, i: AllocateCode) -> Allocating {
        i.events.push(A_Allocate(i.wordlist));
        Allocating { }
    }
    pub fn on_input_code(self, _: InputCode) -> InputtingNameplate {
        // I_Start
        // also, return an Input object
        InputtingNameplate { }
    }
    pub fn on_set_code(self, c: SetCode) -> Known {
        // TODO: try!(validate_code(c.code))
        let code_string = c.code.to_string();
        let nc: Vec<&str> = code_string.splitn(2, '-').collect();
        let _nameplate = Nameplate(nc[0].to_string());
        // N_SetNameplate(nameplate)
        // B_GotCode(code.clone())
        // K_GotCode(code.clone())
        Known { }
    }
}

impl InputtingNameplate {
    pub fn on_got_nameplate(self, _: GotNameplate) -> InputtingWords {
        // N_SetNameplate(_.nameplate)
        InputtingWords { }
    }
}

impl InputtingWords {
    pub fn on_finished_input(self, _: FinishedInput) -> Known {
        // B_GotCode(_.code.clone()), K_GotCode(_.code.clone())
        Known { }
    }
}

impl Allocating {
    pub fn on_allocated(self, _: Known) -> Known {
        // TODO: assert _.code.startswith(_.nameplate+"-")
        // N_SetNameplate(_.nameplate.clone())
        // B_GotCode(_.code.clone())
        // K_GotCode(_.code.clone())
        Known { }
    }
}

pub struct CodeMachine {
    state: CodeStateMachine,
}

impl CodeMachine {
    pub fn new() -> CodeMachine {
        CodeMachine { state: CodeMachine::Idle(Idle{}) }
    }

    pub fn process(&mut self, event: CodeEvent) -> Events {
        let mut evs = events![];
        self.state = match event {
            CodeEvent::AllocateCode(wordlist) =>
                self.state.on_allocate_code(AllocateCode { wordlist, events: &evs }),
            CodeEvent::InputCode => self.state.on_input_code(InputCode { events: &evs }),
            CodeEvent::SetCode(code) => self.state.on_set_code(SetCode { code, events: &evs }),
            CodeEvent::Allocated(nameplate, code) => self.state.on_allocated(Allocated { nameplate, code, events: &evs }),
            CodeEvent::GotNameplate(nameplate) => self.state.on_got_nameplate(GotNameplate { nameplate, events: &evs }),
            CodeEvent::FinishedInput(code) => self.state.on_finished_input(FinishedInput { code, events: &evs }),
        };
        evs
    }
}

/*

impl CodeMachine {
    pub fn new() -> CodeMachine {
        CodeMachine { state: State::Idle }
    }

    pub fn process(&mut self, event: CodeEvent) -> Events {
        use self::State::*;
        let (newstate, actions) = match self.state {
            Idle => self.in_idle(event),
            InputtingNameplate => self.in_inputting_nameplate(event),
            InputtingWords => self.in_inputting_words(event),
            Allocating => self.in_allocating(event),
            Known => self.in_known(&event),
        };

        if let Some(s) = newstate {
            self.state = s;
        }

        actions
    }

    fn in_idle(&mut self, event: CodeEvent) -> (Option<State>, Events) {
        use super::events::CodeEvent::*;
        match event {
            AllocateCode(wordlist) => {
                (Some(State::Allocating), events![A_Allocate(wordlist)])
            }
            InputCode => (Some(State::InputtingNameplate), events![I_Start]), // TODO: return Input object
            SetCode(code) => {
                // TODO: try!(validate_code(code))
                let code_string = code.to_string();
                let nc: Vec<&str> = code_string.splitn(2, '-').collect();
                let nameplate = Nameplate(nc[0].to_string());
                (
                    Some(State::Known),
                    events![
                        N_SetNameplate(nameplate.clone()),
                        B_GotCode(code.clone()),
                        K_GotCode(code.clone())
                    ],
                )
            }
            Allocated(..) => panic!(),
            GotNameplate(..) => panic!(),
            FinishedInput(..) => panic!(),
        }
    }

    fn in_inputting_nameplate(
        &mut self,
        event: CodeEvent,
    ) -> (Option<State>, Events) {
        use super::events::CodeEvent::*;
        match event {
            AllocateCode(..) => panic!(),
            InputCode => panic!(),
            SetCode(..) => panic!(),
            Allocated(..) => panic!(),
            GotNameplate(nameplate) => (
                Some(State::InputtingWords),
                events![N_SetNameplate(nameplate)],
            ),
            FinishedInput(..) => panic!(),
        }
    }

    fn in_inputting_words(
        &mut self,
        event: CodeEvent,
    ) -> (Option<State>, Events) {
        use super::events::CodeEvent::*;
        match event {
            AllocateCode(..) => panic!(),
            InputCode => panic!(),
            SetCode(..) => panic!(),
            Allocated(..) => panic!(),
            GotNameplate(..) => panic!(),
            FinishedInput(code) => (
                Some(State::Known),
                events![B_GotCode(code.clone()), K_GotCode(code.clone())],
            ),
        }
    }

    fn in_allocating(&mut self, event: CodeEvent) -> (Option<State>, Events) {
        use super::events::CodeEvent::*;
        match event {
            AllocateCode(..) => panic!(),
            InputCode => panic!(),
            SetCode(..) => panic!(),
            Allocated(nameplate, code) => {
                // TODO: assert code.startswith(nameplate+"-")
                (
                    Some(State::Known),
                    events![
                        N_SetNameplate(nameplate.clone()),
                        B_GotCode(code.clone()),
                        K_GotCode(code.clone())
                    ],
                )
            }
            GotNameplate(..) => panic!(),
            FinishedInput(..) => panic!(),
        }
    }

    fn in_known(&mut self, event: &CodeEvent) -> (Option<State>, Events) {
        use super::events::CodeEvent::*;
        match *event {
            AllocateCode(..) => panic!(),
            InputCode => panic!(),
            SetCode(..) => panic!(),
            Allocated(..) => panic!(),
            GotNameplate(..) => panic!(),
            FinishedInput(..) => panic!(),
        }
    }
}
*/
