mod execute;
mod storage;

use crate::execute::run_code;
use crate::storage::store_code;
use execute::ExecutionResult;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
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
    let file = File::open(file_path).expect("Unable to open file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).ok();
    contents
}

#[get("/problem/<id>")]
fn get(id: &str) -> Json<Problem> {
    let stmt_path = Path::new(DATASET_DIR).join(id).join("stmt.txt");
    let samples_dir = Path::new(DATASET_DIR).join(id).join("samples");

    let mut sample_inputs = Vec::new();
    let mut sample_outputs = Vec::new();

    let mut samples_paths: Vec<_> = std::fs::read_dir(samples_dir)
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    samples_paths.sort_by_key(|dir| dir.path());

    for sample_path in samples_paths {
        let path = sample_path.path();
        let contents = read_contents(&path);
        match path.display().to_string().split('.').last() {
            Some("in") => sample_inputs.push(contents),
            Some("out") => sample_outputs.push(contents),
            _ => panic!(),
        }
    }

    Json(Problem {
        statement: read_contents(&stmt_path),
        sample_inputs,
        sample_outputs,
    })
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ProgrammingLanguage {
    Cpp,
    Python,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Solution {
    problem_id: String,
    code: String,
    language: ProgrammingLanguage,
}

#[post("/execute", format = "json", data = "<user_input>")]
fn execute_code_on_samples(user_input: Json<Solution>) -> Json<ExecutionResult> {
    let file_path = store_code(&user_input.language, &user_input.code);
    return run_code(user_input, &file_path.unwrap()).unwrap();
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get, execute_code_on_samples])
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
        assert!(problem_count > 0);
    }

    #[test]
    fn all_problems_validation() {
        let paths = std::fs::read_dir(DATASET_DIR).unwrap();

        for path in paths {
            let problem_path = path.unwrap().path().display().to_string();
            let name = problem_path.split('/').last().unwrap();
            dbg!(&name);
            let resp = get(&name);
            assert!(resp.statement.len() > 0);
            assert_eq!(resp.sample_inputs.len(), resp.sample_outputs.len());
        }
    }
}
