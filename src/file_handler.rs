use std::fs;

pub fn list_all_files_in_directory(dir: String) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            println!("{:?}", path.to_str().unwrap());
            files.push(path.to_str().unwrap().to_string());
        }
    }
    files
}
