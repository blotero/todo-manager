use std::fs::OpenOptions;
use std::io::{Result, Write};

pub(crate) struct DataRecord<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub is_done: bool,
    pub priority: u8,
}

impl<'a> DataRecord<'a> {
    pub(crate) fn save(&self) -> Result<usize> {
        println!("Saving record in db...");
        let str_record: String = format!(
            "{}, {}, {}, {}\n",
            self.name, self.description, self.is_done as i32, self.priority,
        );

        println!("Saving: {}", str_record);

        let mut data_file;
        match OpenOptions::new().append(true).open("data/data.txt") {
            Ok(file) => data_file = file,
            Err(err) => return Err(err),
        }
        return data_file.write(str_record.as_bytes());
    }
}
