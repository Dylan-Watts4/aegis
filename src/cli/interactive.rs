use std::io;
use std::io::{Read, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc};
use std::thread;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal;

use crate::core::loghandler::LogHandler;
use crate::core::session::SessionManager;
use crate::cli::output::{send_command, read_output};

pub fn interact_with_session(session_manager: Arc<SessionManager>, session: crate::core::session::Session) -> io::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let kill = Arc::new(AtomicBool::new(false));
    let session_arc = Arc::new(session);

    let running_clone = Arc::clone(&running);
    let session_clone = Arc::clone(&session_arc);

    // Render thread
    thread::spawn(move || {
        let mut buffer = [0u8; 4096];
        while running_clone.load(Ordering::SeqCst) {
            if let Ok(n) = read_output(&session_clone, &mut buffer) {
                if n > 0 {
                    print!("{}", String::from_utf8_lossy(&buffer[..n]));
                    io::stdout().flush().unwrap();
                }
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    LogHandler::success(">> Interactive mode started. Ctrl+B to background, Ctrl+K to kill session.");

    terminal::enable_raw_mode()?;
    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                let send_key = |bytes: &[u8]| {
                    let s = String::from_utf8_lossy(bytes);
                    let _ = send_command(&session_arc, &s);
                };
                match (code, modifiers) {
                    (KeyCode::Char('b'), KeyModifiers::CONTROL) => {
                        LogHandler::info("\n[*] Backgrounding session...");
                        break;
                    }
                    (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                        LogHandler::info("[*] Killing session...");
                        kill.store(true, Ordering::SeqCst);
                        break;
                    }
                    (KeyCode::Tab, _) => send_key(&[b'\t']),
                    (KeyCode::Left, _) => send_key(b"\x1b[D"),
                    (KeyCode::Right, _) => send_key(b"\x1b[C"),
                    (KeyCode::Up, _) => send_key(b"\x1b[A"),
                    (KeyCode::Down, _) => send_key(b"\x1b[B"),
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => send_key(&[0x03]),
                    (KeyCode::Char('z'), KeyModifiers::CONTROL) => send_key(&[0x1A]),
                    (KeyCode::Char(c), _) => send_key(&[c as u8]),
                    (KeyCode::Enter, _) => send_key(b"\n"),
                    (KeyCode::Backspace, _) => send_key(&[0x7f]),
                    (KeyCode::Esc, _) => {
                        LogHandler::info("\n[*] Exiting interactive mode...");
                        break;
                    }
                    _ => {}
                }
            }
        }
        if !running.load(Ordering::SeqCst) {
            break;
        }
    }

    terminal::disable_raw_mode()?;
    running.store(false, Ordering::SeqCst);

    if kill.load(Ordering::SeqCst) {
        session_manager.remove(session_arc.id).ok();
    }

    Ok(())
}