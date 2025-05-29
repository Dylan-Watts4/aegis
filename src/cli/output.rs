pub fn send_command_and_print_output(locked: &mut std::net::TcpStream, command: &str) -> std::io::Result<()> {
    use std::io::{Read, Write};
    use std::thread;
    use std::time::Duration;

    locked.write_all(command.as_bytes())?;
    locked.flush()?;

    thread::sleep(Duration::from_millis(200));

    let mut buf = [0u8; 4096];
    let n = locked.read(&mut buf)?;
    if n > 0 {
        print!("\r\x1b[2K");
        print!("{}", String::from_utf8_lossy(&buf[..n]));
    }

    thread::sleep(Duration::from_millis(100));
    Ok(())
}

pub fn send_command_and_get_output_until(locked: &mut std::net::TcpStream, command: &str, marker: &str) -> std::io::Result<Vec<u8>> {
    use std::io::{Read, Write};
    use std::time::{Duration, Instant};

    locked.write_all(command.as_bytes())?;
    locked.flush()?;

    let mut buf = Vec::new();
    let mut tmp = [0u8; 4096];
    let start = Instant::now();
    let mut marker_count = 0;
    
    loop {
        locked.set_read_timeout(Some(Duration::from_millis(300)))?;
        match locked.read(&mut tmp) {
            Ok(0) => {
                break;
            }, // EOF
            Ok(n) => {
                buf.extend_from_slice(&tmp[..n]);
                marker_count += String::from_utf8_lossy(&tmp[..n])
                    .matches(marker)
                    .count();
                if marker_count >= 2 {
                    break;
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock || e.kind() == std::io::ErrorKind::TimedOut => {
                if start.elapsed() > Duration::from_secs(30) {
                    break;
                }
                continue;
            }
            Err(e) => return Err(e),
        }
        if start.elapsed() > Duration::from_secs(30) {
            break;
        }
    }

    Ok(buf)
}