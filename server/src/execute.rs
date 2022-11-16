use crate::ProgrammingLanguage::*;
use crate::{get, Solution};
use anyhow::Result;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use serde::Serialize;
use std::io::Write;
use std::process::{Command, Stdio};

const EXECUTION_DIR: &'static str = "execution/";

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ExecutionResult {
    submission: Solution,
    inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub submission_outputs: Vec<String>,
    pub errors: Vec<String>,
}

pub fn run_code_on_samples(
    user_input: Json<Solution>,
    _file_path: &str,
) -> Result<Json<ExecutionResult>> {
    let _cmd_cpp_run = format!("{}/main", EXECUTION_DIR);

    let problem = get(&user_input.problem_id);

    let mut res: ExecutionResult = ExecutionResult {
        submission: user_input.clone().into_inner(),
        inputs: problem.sample_inputs.clone(),
        outputs: problem.sample_outputs.clone(),
        submission_outputs: Vec::new(),
        errors: Vec::new(),
    };

    match user_input.language {
        Cpp => {
            let compile = Command::new("g++")
                .arg("submissions/main.cpp")
                .arg("-o")
                .arg("execution/main")
                .output()
                .expect("compile command failed");
            if !compile.stderr.is_empty() {
                res.errors.push(String::from_utf8(compile.stderr)?);
                return Ok(Json(res));
            }

            for input in problem.sample_inputs.clone() {
                let mut child = Command::new("execution/main")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()?;

                let mut stdin = child.stdin.take().unwrap();
                // stdin.write_all(input.as_bytes()).ok();
                // let output = child.wait_with_output()?.stdout;
                std::thread::spawn(move || {
                    stdin.write_all(input.as_bytes()).ok();
                });
                let output = child.wait_with_output()?.stdout;
                let output = String::from_utf8(output).expect("Found invalid UTF-8 in output");
                res.submission_outputs.push(output);
            }
        }
        Python => {
            todo!()
        }
    }

    Ok(Json(res))
}
