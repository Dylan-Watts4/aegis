pub mod completion;
pub mod interactive;
pub mod command;
pub mod output;

pub use command::handle_command;
pub use completion::AegisCompleter;
pub use interactive::interact_with_session;
pub use output::send_command_and_print_output;

use rustyline::error::ReadlineError;
use rustyline::{Editor, history::FileHistory};

pub fn start_repl(session_manager: std::sync::Arc<crate::core::session::SessionManager>, comms_manager: std::sync::Arc<std::sync::Mutex<crate::core::comms::manager::CommsManager>>) {
    let mut rl = Editor::<AegisCompleter, FileHistory>::new().unwrap();
    rl.set_helper(Some(AegisCompleter));

    loop {
        let readline = rl.readline("aegis> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                handle_command(line.trim(), &session_manager, &comms_manager);
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
}