use lazy_static::lazy_static;
use std::{env, fs};
use fancy_regex::Regex;
use std::fs::{File};
use std::io::{Read, Write};
use std::io::BufReader;


const CONF_FILE: &str = "conf.txt";

fn collect_exceptions() -> String {
    let file_name = CONF_FILE;
    let path = env::current_dir();
    let full_path = path.as_ref().unwrap().to_str().unwrap().to_owned();
    let ff= [full_path, "/".to_string(), file_name.to_string()].join("");
    let file = File::open(ff);

    let _greeting_file = match file {
        Ok(_) => (),
        Err(_) => panic!("Отсутствует конфигурационный файл.\nДобавь conf.txt в корень директории с перечислением исключений.\n\
        Синтаксис:\n`com.org\nru.com\n...`"),
    };


    let mut buf_reader = BufReader::new(file.unwrap());
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("");

    lazy_static! {
        static ref HASHTAG_REGEX: Regex = Regex::new(
                r"(?m)([\w\d]+\.[\w\d]+)",
            ).unwrap();
    }
    let result = HASHTAG_REGEX.find_iter(contents.as_str()).map(|mat| mat.unwrap().as_str()).collect::<Vec<_>>();

    let mut ee: Vec<String> = Vec::new();
    for r in result {
        ee.push(format!(r"(?<!(.{}))", r));
    }
    return ee.join("")
    // let ret = ee.join("").replace(".", r"\.");
    // return ret
}

fn remove_sub(domains: &str) -> String {
    let exceptions = collect_exceptions();

    let hashtag_regex: Regex = Regex::new(
        format!(r"(?m)([\w\d]+\.[\w\d]+$){}", exceptions.as_str()).as_str()
    ).unwrap();

    let result = hashtag_regex.find_iter(domains).map(|mat| mat.unwrap().as_str()).collect::<Vec<_>>();
    let new_result = result.join("\n");
    return new_result.to_string()
}

fn remove_file_format (file_name: &mut String) -> String {
    let re: Regex = Regex::new(r".txt$").unwrap();
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
    let path = env::current_dir();
    let re = Regex::new(format!(r".txt(?<!({}))", CONF_FILE).as_str()).unwrap();

    for f in file_list {
        let mut file_name = f.unwrap().path().file_name().unwrap().to_str().unwrap().to_owned();
        let mut full_path = path.as_ref().unwrap().to_str().unwrap().to_owned();

        if re.is_match(file_name.as_str()).unwrap() {
            println!("Name: {}", file_name);
            process_file(&mut file_name, &mut full_path).expect("AAA");
        };
    }

    // println!("Press any button to exit...");
    // io::stdin().read_line(&mut String::new()).unwrap();
    Ok(())
}
