use crate::storage::SUBMISSIONS_DIR;
use crate::Solution;
use anyhow::Result;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use serde::Serialize;
use std::process::Command;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ExecutionResult {
    submission: Solution,
    inputs: Vec<String>,
    outputs: Vec<String>,
    submission_outputs: Vec<String>,
}

pub fn run_code(user_input: Json<Solution>, file_path: &str) -> Result<Json<ExecutionResult>> {
    // Running the code not implemented yet
    // Command::new("g++")
    //     .current_dir(SUBMISSIONS_DIR)
    //     .arg("main.cpp")
    //     .arg("-o")
    //     .arg("main");

    Ok(Json(ExecutionResult {
        submission: user_input.into_inner(),
        inputs: Vec::new(),
        outputs: Vec::new(),
        submission_outputs: Vec::new(),
    }))
}
