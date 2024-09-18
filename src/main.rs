use std::fs;
use colored::Colorize;

struct LsEntry {
    is_dir: bool,
    name: String,
}

fn main()
{
    let is_color = false;
    let mut entries: Vec<LsEntry> = Vec::new();
    let paths = fs::read_dir("./").unwrap();
    
    // Make Vector containiong all files and dirs 
    for path in paths {
        if path.as_ref().unwrap().path().is_dir() {
            entries.push(LsEntry {
                is_dir: true,
                name: path.unwrap().path().to_str().unwrap().to_string(),
            });
        }
        else {
            entries.push(LsEntry {
               is_dir: false,
               name: path.unwrap().path().to_str().unwrap().to_string(),
            });
        }
    }

    // Print directories first
    for dir in &entries {
        if dir.is_dir {
            if is_color {
                println!("{}\t\t{}", "DIR".yellow(), dir.name.as_str().purple());
            }
            else {
                println!("{}\t\t{}", "DIR", dir.name.as_str());
            }
        }
    }

    // Print files
    for file in entries {
        if !file.is_dir {
            let meta = fs::metadata(&file.name);
            let file_size: f64 = meta.unwrap().len() as f64;
            
            let mut new_file_size: f64 = 0.0;
            let mut unit = "";

            if file_size < 1024.0 {
                new_file_size = file_size;
                unit = "B";
            }
            else if file_size < 1024.0*1024.0 {
                new_file_size = file_size / 1024.0;
                unit = "KB";
            }
            else if file_size < 1024.0*1024.0*1024.0 {
                new_file_size = file_size / (1024.0*1024.0);
                unit = "MB";
            }
            else if file_size < 1024.0*1024.0*1024.0*1024.0 {
                new_file_size = file_size / (1024.0*1024.0*1024.0);
                unit = "GB";
            }
            else if file_size < 1024.0*1024.0*1024.0*1024.0*1024.0  {
                new_file_size = file_size / (1024.0*1024.0*1024.0*1024.0);
                unit = "TB";
            }

            let formated: String;
            if unit == "B" {
                formated = format!("{} {}", new_file_size, unit);
            }
            else {
                formated = format!("{:.2} {}", new_file_size, unit);
            }

            if is_color {
                print!("{}\t", &formated.green());
            }
            else {
                print!("{}\t", &formated);
            }

            if formated.len() < 8 {
                print!("\t");
            }

            if is_color {
                println!("{}", file.name.as_str().blue());
            }
            else {
                println!("{}", file.name.as_str());
            }
        }
    }

}
