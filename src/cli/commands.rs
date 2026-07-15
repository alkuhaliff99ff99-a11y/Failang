use std::process;

pub fn execute(args: &[String]) {
    if args.len() < 2 {
        crate::repl::run_repl();
        return;
    }

    match args[1].as_str() {
        "run" => {
            if args.len() < 3 {
                eprintln!("الاستخدام: fsl run file.fsl");
                process::exit(1);
            }
            crate::cli::run::run_file(&args[2]);
        }

        "test" => {
            crate::cli::test::run_tests();
        }

        "build" => {
            crate::cli::build::build();
        }

        "fmt" => {
            crate::cli::fmt::format();
        }

        "new" => {
            crate::cli::new::create();
        }

        file => {
            crate::cli::run::run_file(file);
        }
    }
}
