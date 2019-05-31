use std::fs::File;
use std::io::Read;

pub struct Rom {
    pub data: Vec<u8>,
}

impl Rom {
    pub fn new(file_name: String) -> Self {
        let mut file = match File::open(file_name) {
            Ok(file) => file,
            Err(..) => panic!("file didnt exist"),
        };
        let mut buffer = Vec::new();
        match file.read_to_end(&mut buffer) {
            Ok(content) => content,
            Err(e) => panic!("{}", e),
        };
        Rom { data: buffer }
    }
}
