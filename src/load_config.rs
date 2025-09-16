use crate::types::Task;
use std::fs::File;
use std::io::BufReader;

pub fn load_config(path: String) -> Result<Vec<Task>, String> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(format!("failed to open config file: {}", e)),
    };
    let reader = BufReader::new(file);

    let tasks: Vec<Task> = match serde_yml::from_reader(reader) {
        Ok(tasks) => tasks,
        Err(e) => return Err(format!("failed to parse config file: {}", e)),
    };

    return Ok(tasks);
}
