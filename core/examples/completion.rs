extern crate magic_wormhole_core;
extern crate rustyline;

use magic_wormhole_core::{default_wordlist, Wordlist};

use rustyline::completion::{extract_word, Completer};

struct CodeCompleter {
    wordlist: Wordlist,
}

static BREAK_CHARS: [char; 1] = [' '];

impl Completer for CodeCompleter {
    fn complete(&self, line: &str, pos: usize) -> rustyline::Result<(usize, Vec<String>)> {
        let (start, word) = extract_word(
            line,
            pos,
            &BREAK_CHARS.iter().cloned().collect(),
        );
        Ok((start,self.wordlist.get_completions(word)))
    }
}

fn main() {
    println!("Receive start");

    let mut rl = rustyline::Editor::new();
    let completer = CodeCompleter {wordlist: default_wordlist(2)};
    rl.set_completer(Some(completer));
    match rl.readline("Enter wormhole code: ") {
        Ok(code) => println!("code is {}", code),
        //Err(rustyline::error::ReadlineError::Interrupted) => continue,
        //Err(rustyline::error::ReadlineError::Eof) => break,
        Err(e) => println!("error: {:?}", e),
    };
}
