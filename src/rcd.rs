use std::{env, net::{TcpStream, TcpListener}, io::{Read, Write},sync::{RwLock}, thread, };
use getopts::Options;

const DEFAULT_LISTEN:&str="localhost";
const DEFAULT_PORT:u16=3928;
const VERSION:&str = "1.0.0";

struct Config{
    listen:String,
    port:u16,
}
fn print_usage(_program:&str,opts:Options){
    let brief:String = String::new();
    opts.usage(&brief);
    println!("{}",&brief);
}
static CLIP_BOARD:RwLock<String> = RwLock::new(String::new());

fn main() {
    /* parse args */
    let args: Vec<String> = env::args().collect();
    let program: String = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("l", "listen", "listen host,default localhost", "NAME");
    opts.optopt("p", "port",   "listen port,default 3298", "NAME");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}",f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let mut config = Config {
        listen: DEFAULT_LISTEN.to_string(),
        port: DEFAULT_PORT
    };
    let listen = matches.opt_str("l");
    if listen.is_some() {
        config.listen = listen.unwrap().to_string();
    }
    let port = matches.opt_get::<u16>("l").unwrap();
    if port.is_some() {
        config.port = port.unwrap();
    }


    let listener = TcpListener::bind(format!("{}:{}",config.listen,config.port)).unwrap();
    println!("Listening for connections on {}:{}", config.listen,config.port);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle(stream);
                });
             }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}

fn handle(mut stream:TcpStream) {
    /* 
     * a mtu usually 1400, malloc a block a little large than it
     * also this program is not support a clipboard which size > 1400
     */
    let mut input = [0;1500];
    while let Ok(size) = stream.read(&mut input){
        if size == 0{
            /* client close tcp */
            break;
        }
        let line = String::from_utf8_lossy(&input[0..size]);
        if let Some((cmd,value)) = line.split_once(':') {
            if cmd.eq("c") {
                /* copy */
                if let Ok(mut res) = CLIP_BOARD.write() {
                    *res = String::from(value);
                }
                stream.write_all("ok".as_bytes()).unwrap();
                break;
            }
            else if cmd.eq("p") {
                /* paste */
                if let Ok(base64_value) = CLIP_BOARD.read(){
                    println!("{}",base64_value);
                    stream.write_all(base64_value.as_bytes()).unwrap();
                }
                break;
            }
            else if cmd.eq("v") {
                stream.write_all(VERSION.as_bytes()).unwrap();
            }
        }
    }
}