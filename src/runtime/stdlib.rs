use std::fs;
use std::path::Path;

use crate::compiler::interpreter::Interpreter;
use crate::repl::execute;

pub fn load_stdlib(interpreter: &mut Interpreter) {
    let path = Path::new("stdlib");

    if let Ok(entries) = fs::read_dir(path) {
        let mut files: Vec<String> = entries
            .flatten()
            .filter_map(|entry| {
                let path = entry.path();

                if path.extension()? == "fsl" {
                    Some(path.to_string_lossy().to_string())
                } else {
                    None
                }
            })
            .collect();

        files.sort();

        for file in files {
            if let Ok(source) = fs::read_to_string(&file) {
                execute(&source, interpreter);
            }
        }
    }
}
