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

pub fn interact_with_session(session_manager: Arc<SessionManager>, session: crate::core::session::Session) -> io::Result<()> {
    let stream = session.stream.clone();
    let running = Arc::new(AtomicBool::new(true));
    let kill = Arc::new(AtomicBool::new(false));

    let running_clone = Arc::clone(&running);
    //let kill_clone = Arc::clone(&kill);
    let stream_clone = Arc::clone(&stream);

    // Render thread
    thread::spawn(move || {
        let mut buffer = [0; 1024];
        while running_clone.load(Ordering::SeqCst) {
            let n = {
                let mut stream = stream_clone.lock().unwrap();
                match stream.read(&mut buffer) {
                    Ok(n) => Ok(n),
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(0),
                    Err(e) => Err(e),
                }
            };
            if let Ok(n) = n {
                if n > 0 {
                    //print!("\r\x1b[2K");
                    print!("{}", String::from_utf8_lossy(&buffer[..n]));
                    io::stdout().flush().unwrap();
                }
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    LogHandler::success(">> Interactive mode started. Ctrl+B to background, Ctrl+K to kill session.");

    terminal::enable_raw_mode()?;
    let mut input_buffer = String::new();
    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                match (code, modifiers) {
                    (KeyCode::Char('b'), KeyModifiers::CONTROL) => {
                        // Ctrl+B
                        LogHandler::info("\n[*] Backgrounding session...");
                        break;
                    }
                    (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                        // Ctrl+K
                        LogHandler::info("[*] Killing session...");
                        kill.store(true, Ordering::SeqCst);
                        break;
                    }
                    (KeyCode::Tab, _) => {
                        let mut locked = stream.lock().unwrap();
                        locked.write_all(&[b'\t'])?;
                        locked.flush()?;
                    }
                    (KeyCode::Left, _) => {
                        let mut locked = stream.lock().unwrap();
                        locked.write_all(b"\x1b[D")?;
                        locked.flush()?;
                    }
                    (KeyCode::Right, _) => {
                        let mut locked = stream.lock().unwrap();
                        locked.write_all(b"\x1b[C")?;
                        locked.flush()?;
                    }
                    (KeyCode::Up, _) => {
                        let mut locked = stream.lock().unwrap();
                        locked.write_all(b"\x1b[A")?;
                        locked.flush()?;
                    }
                    (KeyCode::Down, _) => {
                        let mut locked = stream.lock().unwrap();
                        locked.write_all(b"\x1b[B")?;
                        locked.flush()?;
                    }
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                        let mut locked = stream.lock().unwrap();
                        locked.write_all(&[0x03])?; // Ctrl+C
                        locked.flush()?;
                    }
                    (KeyCode::Char('z'), KeyModifiers::CONTROL) => {
                        let mut locked = stream.lock().unwrap();
                        locked.write_all(&[0x1A])?; // Ctrl+Z
                        locked.flush()?;
                    }
                    (KeyCode::Char(c), _) => {
                        /*
                        input_buffer.push(c);
                        print!("{}", c);
                        io::stdout().flush().unwrap();
                        */
                        let mut locked = stream.lock().unwrap();
                        locked.write_all(&[c as u8])?;
                        locked.flush()?;
                    }
                    (KeyCode::Enter, _) => {
                        /*
                        input_buffer.push('\n');
                        let mut locked = stream.lock().unwrap();
                        locked.write_all(input_buffer.as_bytes())?;
                        locked.flush()?;
                        input_buffer.clear();
                        print!("\r\n");
                        io::stdout().flush().unwrap();
                        */
                        let mut locked = stream.lock().unwrap();
                        locked.write_all(b"\n")?;
                        locked.flush()?;
                    }
                    (KeyCode::Backspace, _) => {
                        /*
                        input_buffer.pop();
                        print!("\x08 \x08");
                        io::stdout().flush().unwrap();
                        */
                        let mut locked = stream.lock().unwrap();
                        locked.write_all(&[0x7f])?; // DEL
                        locked.flush()?;
                    }
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
        session_manager.remove(session.id).ok();
    }

    Ok(())
}