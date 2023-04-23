use lazy_static::lazy_static;
use std::{env, fs};
use regex::{Regex};
use std::fs::{File};
use std::io::{Read, Write};
use std::io::BufReader;


fn remove_sub(domains: &str) -> String {
    lazy_static! {
        static ref HASHTAG_REGEX: Regex = Regex::new(
                r"(?m)[\w\d]+\.[\w\d]+[^\.]$",
            ).unwrap();
    }
    let result = HASHTAG_REGEX.find_iter(domains).map(|mat| mat.as_str()).collect::<Vec<_>>();
    let new_result = result.join("\n");
    return new_result.to_string()
}

fn remove_file_format (file_name: &mut String) -> String {
    let re = Regex::new(r".txt$").unwrap();
    let result = re.replace_all(file_name, "");
    return result.to_string()
}

fn process_file(file_name: &mut String, full_path: &mut String) -> std::io::Result<()> {
    // Open and read contents
    let file = File::open([full_path, "/", file_name].join(""))?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    if contents.contains(".com") || contents.contains(".net") || contents.contains(".ru") {
        // Remove sub
        let result = remove_sub(contents.as_mut_str());

        // Create files and save
        let result_path = [full_path, "/", "result"].join("");
        fs::create_dir_all(&result_path).expect("Couldn’t read from stdin");
        let file_name_clean = remove_file_format(file_name);
        let result_file = [&result_path, "/", &file_name_clean, "new.txt"].join("");
        let mut filer = File::create(result_file)?;
        filer.write_all(result.as_bytes())?;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let path = env::current_dir().unwrap();
    let file_list = fs::read_dir(path).unwrap();
    let path2 = env::current_dir();

    for f in file_list {
        let mut file_name = f.unwrap().path().file_name().unwrap().to_str().unwrap().to_owned();
        let mut full_path = path2.as_ref().unwrap().to_str().unwrap().to_owned();
        let re = Regex::new(r".txt").unwrap();

        if re.is_match(file_name.as_str()) {
            // let r = f.unwrap().path().display();
            println!("Name: {}", file_name);
            process_file(&mut file_name, &mut full_path).expect("AAA");
        };
    }
    Ok(())
}
