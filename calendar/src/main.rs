


use std::io::*;
use std::fs::*;
use std::env;
use chrono::prelude::*;
use std::ffi::OsStr;
use std::path::Path;
use std::time::SystemTime;
use chrono::{DateTime, Local, Utc};

mod constants;
mod logic;

use constants::constant::LogParam;


fn main() {

    let args: Vec<String> = env::args().collect();
    let args1_string = args[1].to_string();
    let directories: Vec<_> = args1_string.split(',').collect();

    println!("{:?}", directories);
    
    let mut my_logParam: LogParam = LogParam::new();
    
    my_logParam.set_directory(args[1].to_string());
    println!("{}", my_logParam.get_directory());

    
    for dirInfo in directories.iter() {
       let from_path = Path::new(dirInfo);
       let dir = read_dir(from_path).unwrap();
       let hello = dir.map(|res| res.map(|e| e.path())).collect::<Result<Vec<_>>>().unwrap();

        for h in hello.iter() {
            let x = h.to_owned();

            if !x.is_dir() {

                let xyy: String = x.file_name().unwrap().to_str().unwrap().into();               
                println!("{}", xyy);

                //if check_log(xyy) {


                //}
            }
        }
    }

}
