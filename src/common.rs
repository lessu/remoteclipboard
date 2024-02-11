use std::{path::Path, fs::File, io::{Write, BufReader, BufRead}};

const DEFAULT_HOST:&str="localhost";
const DEFAULT_PORT:u16=3928;

pub struct Config{
    pub host:String,
    pub port:u16,
}

pub fn dump_config(config:&Config) -> String{
    format!("host={}\nport={}\n",config.host,config.port)
}

pub fn read_config(profile_path:&str,is_default_path:bool) -> Config{
    let mut config = Config {
        host:DEFAULT_HOST.to_string(),
        port:DEFAULT_PORT
    };

    if is_default_path && !Path::new(profile_path).exists(){
        // create default one
        let mut file = File::create(profile_path).unwrap();
        file.write_all( 
            dump_config(&config).as_bytes() 
        ).unwrap();
    }
    let file = File::open(profile_path).unwrap();
    let reader = BufReader::new(file);


    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if let Some((key,value)) = line.split_once("="){
            if key == "host" {
                config.host = value.to_string();
            }
            else if key == "port" {
                config.port = value.parse().unwrap();
            }
            else{
                eprintln!("unknown config key {}",key);
            }
        }
    }
    println!("config\n{}",dump_config(&config));
    config

}
