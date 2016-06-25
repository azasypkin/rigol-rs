#![feature(plugin)]

#![plugin(docopt_macros)]

extern crate docopt;
extern crate encoding;
#[macro_use]
extern crate hyper;
extern crate iron;
#[macro_use]
extern crate log;
extern crate mount;
extern crate router;
extern crate rustc_serialize;
extern crate staticfile;

use encoding::{ Encoding, EncoderTrap };
use encoding::all::ASCII;

use hyper::mime::{Mime, TopLevel, SubLevel};

use iron::prelude::*;
use iron::{ Handler };
use iron::headers::ContentType;
use iron::status::Status;

use std::io::prelude::*;
use std::io::{ BufReader };
use std::net::{ ToSocketAddrs, TcpStream };

use std::path::Path;

use staticfile::Static;
use mount::Mount;

docopt!(Args derive Debug, "
Rigol DS1000Z management command line tool.

Usage: rigol-rs [-v] [-h] [-a <address>] [-p <port>] [-i <iface>]

Options:
    -v, --verbose            Toggle verbose output.
    -a, --address <address>  Rigol IP address. [default: 192.168.1.12]
    -p, --port <port>        Rigol port number. [default: 5555]
    -i, --iface <iface>      Specify the local IP interface.
    -h, --help               Print this help menu.
",
        flag_address: String,
        flag_port: u16,
        flag_iface: Option<String>,
flag_config: Option<Vec<String>>);

fn perform_command<A: ToSocketAddrs>(address: A) {
    // let mut command_bytes = ASCII.encode(":CURSor:MANual:YDELta?", EncoderTrap::Strict).unwrap();
    let mut command_bytes = ASCII.encode(":RUN", EncoderTrap::Strict).unwrap();
    command_bytes.push('\r' as u8);
    let mut stream = TcpStream::connect(address).unwrap();
    stream.write_all(&command_bytes).unwrap();

    let mut reader = BufReader::new(stream);

    let mut response = String::new();
    reader.read_line(&mut response).unwrap();
    println!("Response: {}", response);
}

fn read_image<A: ToSocketAddrs>(address: A) -> Vec<u8> {
    // let mut command_bytes = ASCII.encode(":CURSor:MANual:YDELta?", EncoderTrap::Strict).unwrap();
    let mut command_bytes = ASCII.encode(":DISP:DATA?", EncoderTrap::Strict).unwrap();
    command_bytes.push('\r' as u8);
    let mut stream = TcpStream::connect(address).unwrap();
    stream.write_all(&command_bytes).unwrap();

    let mut buffer1 = [0; 576032];
    stream.read_exact(&mut buffer1).unwrap();
    println!("{:?}", buffer1.len());

    let mut buffer2 = [0; 576036];
    stream.read_exact(&mut buffer2).unwrap();
    println!("{:?}", buffer2.len());

    println!("{:?}", buffer2.len());

    let mut buffer: Vec<u8> = buffer1[0..].to_vec();
    buffer.extend_from_slice(&buffer2);

    println!("{:?}", buffer.len());

    buffer
}

struct Run {
    address: String,
    port: u16,
}

impl Handler for Run {
    fn handle (&self, _: &mut Request) -> IronResult<Response> {
        //perform_command((self.address.as_ref(), self.port));
        let buffer = read_image((self.address.as_ref(), self.port));
        //println!("Len {}", buffer.len());
        let data: Vec<u8> = buffer[11..].to_vec();

        let mut response = Response::with(data);
        response.status = Some(Status::Ok);
        response.headers.set(ContentType(Mime(TopLevel::Image, SubLevel::Bmp, vec![])));
        Ok(response)
    }
}

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

    println!("Rigol address {}:{}", args.flag_address, args.flag_port);

    let mut mount = Mount::new();

    mount.mount("/", Static::new(Path::new("public/")));
    mount.mount("/run", Run {
        address: args.flag_address,
        port: args.flag_port,
    });

    println!("Server running on http://localhost:3000/");

    Iron::new(mount).http("127.0.0.1:3000").unwrap();
}
