use std::{process::{Command, Stdio}};

const DATASET_DIR: &str = "../dataset/";

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_problems_exist() {
        let problem_list = Command::new("ls")
            .arg(DATASET_DIR)
            .output()
            .expect("Unable to read problem dataset directory");
        let problem_count: usize = problem_list.stdout.split(|ch| u8::to_string(ch) == "\n").count();
        // println!("Problems found: {}", problem_count);
        assert!(problem_count > 0);
    }
}
