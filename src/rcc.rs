use std::{io::Write, env, net::TcpStream, process::ExitCode};
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
    
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return ExitCode::FAILURE;
    };
    
    if let Ok(mut stream) = TcpStream::connect(format!("{}:{}",config.host,config.port)){
        let base64_value = BASE64_STANDARD.encode(input);
        stream.write(format!("c:{}",base64_value).as_bytes()).unwrap();
        return ExitCode::SUCCESS;
    }else{
        eprintln!("fail to connect to server.");
        return ExitCode::FAILURE;
    }


}