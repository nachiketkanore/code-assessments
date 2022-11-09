use crate::ProgrammingLanguage;
use anyhow::Result;
use std::fs::File;
use std::io::Write;

pub const SUBMISSIONS_DIR: &str = "submissions/";
const FILE_NAME: &str = "main";

fn get_extension(lang: &ProgrammingLanguage) -> &'static str {
    match lang {
        ProgrammingLanguage::Cpp => "cpp",
        ProgrammingLanguage::Python => "py",
    }
}

pub fn store_code(lang: &ProgrammingLanguage, code: &String) -> Result<String> {
    let file_path = format!("{}{}.{}", SUBMISSIONS_DIR, FILE_NAME, get_extension(&lang));
    let mut file = File::create(&file_path)?;
    write!(file, "{}", code)?;
    dbg!(&file_path);
    // handled this manually, will find a better way later
    let mut code_path = String::new();
    code_path.push_str(&SUBMISSIONS_DIR);
    code_path.push_str(FILE_NAME);
    code_path.push_str(".");
    code_path.push_str(get_extension(&lang));
    Ok(code_path)
}
