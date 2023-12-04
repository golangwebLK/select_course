use std::fs::File;
use std::io::Read;
use serde::Deserialize;

#[derive(Deserialize,Debug,Clone)]
pub struct Config{
    pub server:Server,
}

#[derive(Deserialize, Debug,Clone)]
pub struct Server{
    pub ip:String,
}
pub fn parsing_conf_toml() -> Config{
    let file_path = "Config.toml";
    let mut file = match File::open(file_path){
        Ok(f)=>f,
        Err(e)=>panic!("no such file {} exception:{}", file_path, e)
    };
    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(_s)=>{},
        Err(e) => panic!("Error Reading file: {}", e)
    }

    toml::from_str(&str_val).unwrap()
}




