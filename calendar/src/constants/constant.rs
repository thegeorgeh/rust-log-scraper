


/*struct LOG_DATA {
    LogName: String,
    ServerName: String,
    DateCaptcha: String,
    NumErrors: u32,
    Error: String,
}*/





#[derive(Default)]
pub struct LogParam {
   directory: String,
   // keywords_list: Vec<String>,
}

impl LogParam {

    pub fn new() -> LogParam {
        LogParam {
            directory: String::from(""),
        }
    }
    
   pub fn set_directory(&mut self, path: String) {
	self.directory = path;
   }

    
    pub fn get_directory(&self) -> &String {
	&self.directory
    }
}
