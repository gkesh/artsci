use glob::glob;
use std::fs;

pub fn read_ars_dir(loc: &str) -> Vec<String> {
  let mut ars_files: Vec<String> = vec![];

  for entry in glob(&format!("{}/*.ars", loc)).expect("No artsci directory in .config folder!") {
    match entry {
      Ok(path) => {
        let path_str: String = path.display().to_string();
        ars_files.push(path_str);
      },
      Err(e) => println!("{:?}", e),
    }
  }

  ars_files
}

pub fn read_ars(file_path: &str) -> String {
  let contents: String = fs::read_to_string(file_path).expect(&format!("Problem with file: {}", file_path));
  
  contents
}