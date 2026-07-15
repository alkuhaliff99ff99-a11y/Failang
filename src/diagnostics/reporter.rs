use super::error::DiagnosticError;

pub fn report(error: &DiagnosticError) {
    println!("{}", error.display());
}
