use std::thread;
use clap::{Arg, App};
use std::process::Command;
use std::io::{Read, Write};
use anyhow::{Result, Error};
use std::net::{TcpListener, TcpStream};

enum ReadHandle {
    Main(std::io::Stdin),
    Child(std::process::ChildStdout)
}

impl std::io::Read for ReadHandle {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Self::Main(s) => s.read(buf),
            Self::Child(cs) => cs.read(buf)
        }
    }
}

enum WriteHandle {
    Main(std::io::Stdout),
    Child(std::process::ChildStdin)
}

impl std::io::Write for WriteHandle {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Self::Main(s) => s.write(buf),
            Self::Child(cs) => cs.write(buf)
        }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Self::Main(s) => s.flush(),
            Self::Child(cs) => cs.flush()
        }
    }
}

// I need this enum to keep the Child process struct around in the handle_conn function
// so that I can split out the handles
enum ProcHandles {
    Main,
    Child(std::process::Child)
}

struct Flags {
    listen: bool,
    quiet: bool
}

fn read_stdinput(mut std_in: ReadHandle, mut stream: TcpStream) {
    loop {
        let mut buf = [0; 4096];
        match std_in.read(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                else {
                    match stream.write_all(&buf[..n]) {
                        Ok(()) => {},
                        Err(e) => { println!("Encountered error sending stream: {}", e); }
                    }
                }
            }
            Err(e) => { println!("Encountered error reading from handle: {}", e); }
        }
    }
}

fn handle_conn(mut stream: TcpStream, cmd: String) -> Result<usize> {
    let handles = if !cmd.is_empty() {
        ProcHandles::Child(Command::new(cmd)
            .stdout(std::process::Stdio::piped())
            .stdin(std::process::Stdio::piped())
            .spawn()
            .expect("Error spawning process")) 
        } else {
            ProcHandles::Main
    };

    // split read and write handles
    let (s_read, mut s_write) = match handles {
        ProcHandles::Child(child) => {
            (ReadHandle::Child(child.stdout.unwrap()),
            WriteHandle::Child(child.stdin.unwrap()))
        }
        ProcHandles::Main => {
            (ReadHandle::Main(std::io::stdin()),
            WriteHandle::Main(std::io::stdout()))
        }
    };

    let streamx = stream.try_clone()?;
    thread::spawn(|| {
        read_stdinput(s_read, streamx);
    });

    loop {
        let mut buf = [0; 4096];
        match stream.read(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    break Ok(0);
                }
                else {
                    s_write.write_all(&buf[..n])?;
                    s_write.flush()?;
                }
            }
            Err(e) => {
                return Err(Error::new(e));
            }
        }
    }
}

fn run(addr_string: String, cmd: String, flags: Flags) -> Result<usize> {
    let stream = if flags.listen {
        let listener = TcpListener::bind(&addr_string)?;
        if !flags.quiet {
            println!("Listening on {}", addr_string);
        }
        let (stream, addr) = listener.accept()?;
        if !flags.quiet {
            println!("Received connection from {}", addr);
        }
        stream
    } else {
        let stream = TcpStream::connect(&addr_string)?;
        if !flags.quiet {
            println!("Connected to {}", addr_string);
        }
        stream
    };

    handle_conn(stream, cmd)
}

fn main() {
    let matches = App::new("Serval")
                .version("1.0.1")
                .arg(Arg::with_name("IP")
                    .requires("PORT"))
                .arg(Arg::with_name("PORT"))
                .arg(Arg::with_name("port")
                    .help("listen on 0.0.0.0 <port>")
                    .short("l")
                    .long("listen")
                    .conflicts_with("IP")
                    .takes_value(true))
                .arg(Arg::with_name("exec")
                    .help("Execute a program and pipe stdin/out to network")
                    .short("e")
                    .long("exec")
                    .takes_value(true))
                .arg(Arg::with_name("quiet")
                    .help("suppress output")
                    .short("q")
                    .long("quiet"))
                .get_matches();
    let mut cmd = String::new();
    if matches.is_present("exec") {
        cmd.push_str(matches.value_of("exec").unwrap());
    }
    let mut flags = Flags { listen: false, quiet: false };
    flags.quiet = matches.is_present("quiet");
    let mut addr_string = String::new();
    if matches.is_present("port") {
        flags.listen = true;
        addr_string.push_str("0.0.0.0");
        addr_string.push(':');
        let port = matches.value_of("port").unwrap();
        addr_string.push_str(port);
    } else if !matches.is_present("IP") || !matches.is_present("PORT") {
            println!("Invalid usage see -h for more information");
    } else {
        let ip = matches.value_of("IP").unwrap();
        addr_string.push_str(ip);
        addr_string.push(':');
        let port = matches.value_of("PORT").unwrap();
        addr_string.push_str(port);
    }

    match run(addr_string, cmd, flags) {
        Ok(_n) => {},
        Err(e) => { println!("Error {:?}", e) }
    }

}
