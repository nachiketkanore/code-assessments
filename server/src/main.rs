use rocket::serde::json::Json;
use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[macro_use]
extern crate rocket;

const DATASET_DIR: &str = "dataset/";

#[get("/")]
fn index() -> &'static str {
    return "Welcome to code-assessments";
}

#[derive(Serialize)]
struct Problem {
    statement: String,
    sample_inputs: Vec<String>,
    sample_outputs: Vec<String>,
}

fn read_contents(file_path: &Path) -> String {
    dbg!(file_path);
    let file = File::open(file_path).expect("Unable to open file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).ok();
    contents
}

#[get("/problem/<id>")]
fn get(id: &str) -> Json<Problem> {
    let stmt_path = Path::new(DATASET_DIR).join(id).join("stmt.txt");
    let problem = Problem {
        statement: read_contents(&stmt_path),
        sample_inputs: Vec::new(),
        sample_outputs: Vec::new(),
    };
    Json(problem)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn check_problems_exist() {
        let problem_list = Command::new("ls")
            .arg(DATASET_DIR)
            .output()
            .expect("Unable to read problem dataset directory");
        let problem_count: usize = problem_list
            .stdout
            .split(|ch| u8::to_string(ch) == "\n")
            .count();
        // println!("Problems found: {}", problem_count);
        assert!(problem_count > 0);
    }
}
