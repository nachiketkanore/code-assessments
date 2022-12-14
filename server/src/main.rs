mod execute;
mod storage;

use crate::execute::run_code_on_samples;
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

#[derive(Clone, Serialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ProgrammingLanguage {
    Cpp,
    Python,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Solution {
    problem_id: String,
    code: String,
    language: ProgrammingLanguage,
}

#[post("/execute", format = "json", data = "<user_input>")]
fn execute_code_on_samples(user_input: Json<Solution>) -> Json<ExecutionResult> {
    let file_path = store_code(&user_input.language, &user_input.code);
    dbg!(&file_path);
    return run_code_on_samples(user_input, &file_path.unwrap()).expect("code running failed");
}

#[launch]
fn rocket() -> _ {
    let path = std::env::current_dir().unwrap();
    dbg!("The current directory is {}", path.display());
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

    #[test]
    fn code_saving_and_reading() {
        let code="#include<iostream>\n  int main() { int a, b; std::cin >> a >> b; std::cout << (a|b) << std::endl; return 0;}".to_string();
        let language = ProgrammingLanguage::Cpp;
        let code_path = store_code(&language, &code).unwrap();
        let contents =
            std::fs::read_to_string(code_path).expect("Unable to read the contents of file");
        assert_eq!(code, contents);
    }

    #[test]
    fn code_execution() {
        // checks on problem `001` for successful compilation
        // and expected outputs on running against samples
        let submit = Solution {
            problem_id: "001".to_string(),
            code: "#include<iostream>\n  int main() { int a, b; std::cin >> a >> b; std::cout << (a|b) << std::endl; return 0;}".to_string(),
            language: ProgrammingLanguage::Cpp
        };

        let code_path = store_code(&submit.language, &submit.code).unwrap();
        let res = run_code_on_samples(Json(submit), &code_path)
            .unwrap()
            .into_inner();

        assert_eq!(res.errors.len(), 0);
        assert_eq!(res.submission_outputs, res.outputs);
    }
}
