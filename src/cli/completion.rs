use rustyline::completion::{Completer, Pair};
use rustyline::hint::Hinter;
use rustyline::highlight::Highlighter;
use rustyline::validate::Validator;
use rustyline::{Helper, Context};

pub struct AegisCompleter;

impl Completer for AegisCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        _pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), rustyline::error::ReadlineError> {
        let commands = [
            "help", "exit", "listen", "sessions", "close", "interact",
            "upgrade-shell", "run-script", "modules", "run-module"
        ];
        let mut completions = Vec::new();
        let start = 0;
        for cmd in commands.iter() {
            if cmd.starts_with(line) {
                completions.push(Pair {
                    display: cmd.to_string(),
                    replacement: cmd.to_string(),
                });
            }
        }
        Ok((start, completions))
    }
}

impl Hinter for AegisCompleter {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> {
        None
    }
}
impl Highlighter for AegisCompleter {}
impl Validator for AegisCompleter {}
impl Helper for AegisCompleter {}