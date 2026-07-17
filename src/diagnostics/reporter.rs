use super::error::DiagnosticError;

pub struct DiagnosticReporter;

impl DiagnosticReporter {
    pub fn report(error: &DiagnosticError) {
        println!("[FSL Error]");
        println!();

        println!("عربي:");
        println!("{}", error.arabic);
        println!();

        println!("English:");
        println!("{}", error.english);
    }
}

pub fn report(error: &DiagnosticError) {
    DiagnosticReporter::report(error);
}
