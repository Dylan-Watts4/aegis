// TODO: Add timeouts to all functions to allow for safe communications between servers with high ping

use std::io::{self, ErrorKind, Read, Write};
use std::time::{Duration, Instant};
use crate::core::session::{Session, ProtocolStream};
use crate::core::loghandler::LogHandler;

// Send a command to a session, protocol-agnostic
pub fn send_command(session: &Session, command: &str) -> std::io::Result<()> {
    match &session.stream {
        ProtocolStream::Tcp(stream) => {
            let mut locked = stream.lock().unwrap();
            locked.write_all(command.as_bytes())?;
            locked.flush()?; // <-- Ensure the command is sent immediately
        }
        ProtocolStream::Udp { socket, remote_addr } => {
            socket.send_to(command.as_bytes(), remote_addr)?;
        }
    }
    Ok(())
}

// Read output from session, protocol-agnostic
// TODO: filter by remote_addr
pub fn read_output(session: &Session, buf: &mut [u8]) -> io::Result<usize> {
    match &session.stream {
        ProtocolStream::Tcp(stream) => {
            let mut locked = stream.lock().unwrap();
            match locked.read(buf) {
                Ok(n) => Ok(n),
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => Ok(0),
                Err(e) => Err(e),
            }
        }
        ProtocolStream::Udp { socket, remote_addr: _ } => {
            socket.recv(buf)
        }
    }
}

// Send a command and print output
pub fn send_command_and_print_output(session: &Session, command: &str) -> std::io::Result<()> {
    send_command(session, command)?;
    let mut buf = [0u8; 4096];
    let overall_start = Instant::now();
    let idle_timeout = Duration::from_millis(200); // How long to wait after last data
    let max_timeout = Duration::from_secs(3);      // Max total time to wait
    let mut last_data = Instant::now();

    loop {
        let n = read_output(session, &mut buf)?;
        if n > 0 {
            print!("{}", String::from_utf8_lossy(&buf[..n]));
            last_data = Instant::now();
        } else {
            std::thread::sleep(Duration::from_millis(50));
        }
        if last_data.elapsed() > idle_timeout || overall_start.elapsed() > max_timeout {
            break;
        }
    }
    Ok(())
}

// Send a command and read output until a marker is seen
pub fn send_command_and_get_output_until(session: &Session, command: &str, marker: &str) -> std::io::Result<Vec<u8>> {
    send_command(session, command)?;
    let mut buf = Vec::new();
    let mut tmp = [0u8; 4096];
    let start = Instant::now();
    let marker_bytes = marker.as_bytes();
    let mut marker_count = 0;

    loop {
        let n = read_output(session, &mut tmp)?;
        if n > 0 {
            buf.extend_from_slice(&tmp[..n]);
            // Count marker occurrences in the new data
            marker_count += tmp[..n]
                .windows(marker_bytes.len())
                .filter(|window| *window == marker_bytes)
                .count();
            if marker_count >= 2 {
                break;
            }
        } else {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        if start.elapsed() > Duration::from_secs(30) {
            LogHandler::warn("Timeout waiting for end marker");
            break;
        }
    }

    Ok(buf)
}

// Send data in chunks (useful for uploads)
pub fn send_chunks(session: &Session, data: &[u8], chunk_size: usize, prefix: &str, suffix: &str) -> std::io::Result<()> {
    for chunk in data.chunks(chunk_size) {
        let chunk_str = std::str::from_utf8(chunk).unwrap_or("");
        let cmd = format!("{}{}{}\n", prefix, chunk_str, suffix);
        send_command(session, &cmd)?;
    }
    Ok(())
}

// Read all output until timeout (useful for long running scripts)
pub fn read_all_output(session: &Session, timeout_secs: u64) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut tmp = [0u8; 4096];
    let start = Instant::now();
    loop {
        let n = read_output(session, &mut tmp)?;
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&tmp[..n]);
        if start.elapsed() > Duration::from_secs(timeout_secs) {
            break;
        }
    }
    Ok(buf)
}

// Send command until marker and print
pub fn send_command_and_print_output_until_marker(session: &Session, command: &str, marker: &str) -> std::io::Result<()> {
    send_command(session, command);
    let mut buf = [0u8; 4096];
    let marker_bytes = marker.as_bytes();
    let mut window = Vec::new();
    let start = Instant::now();
    let timeout = Duration::from_secs(600);

    loop {
        let n = read_output(session, &mut buf)?;
        if n > 0 {
            print!("{}", String::from_utf8_lossy(&buf[..n]));
            window.extend_from_slice(&buf[..n]);
            if window.len() > marker_bytes.len() * 2 {
                window.drain(..window.len() - marker_bytes.len() * 2);
            }
            if window.windows(marker_bytes.len()).filter(|w| *w == marker_bytes).count() >= 2 {
                break;
            }
        } else {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        if start.elapsed() > timeout {
            LogHandler::warn("Timeout waiting for end marker");
            break;
        }
    }
    Ok(())
}