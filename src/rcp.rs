use std::{io::{Write,Read}, env,net::TcpStream, process::ExitCode};
use getopts::Options;
use base64::{self, prelude::BASE64_STANDARD, Engine};
mod common;
use crate::common::read_config;


fn print_usage(_program:&str,opts:Options){
    let brief:String = String::new();
    opts.usage(&brief);
    println!("{}",&brief);
}


fn main() -> ExitCode{
    let home_path = env!("HOME");
    let mut profile_path = [home_path,"/.rclip_profile"].join("");

    /* parse args */
    let args: Vec<String> = env::args().collect();
    let program: String = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("p", "profile", "set profile file path", "FILE");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}",f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return ExitCode::FAILURE;
    }
    let mut is_default_profile_path = true;
    let profile = matches.opt_str("p");
    if profile.is_some() {
        profile_path = profile.unwrap();
        is_default_profile_path = false;
    }
    let config = read_config(&profile_path,is_default_profile_path);
    
    if let Ok(mut stream) = TcpStream::connect(format!("{}:{}",config.host,config.port)){
        let mut output = [0; 4096];
        stream.write("p:".as_bytes()).unwrap();
        let size = stream.read(&mut output).unwrap();
        let output_str = String::from_utf8(output[0..size].to_vec()).unwrap();
        match BASE64_STANDARD.decode(output_str) {
            Ok(content) => {
                println!("{}",String::from_utf8(content).unwrap());
                return ExitCode::SUCCESS;
            }
            Err(e) => {
                eprintln!("bad content from server,{}",e);
                return ExitCode::FAILURE;
            }
        }
        
    }else{
        eprintln!("failed to connect with server");
        return ExitCode::FAILURE;
    }


}