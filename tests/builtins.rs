use fsl::compiler::interpreter::Interpreter;
use fsl::compiler::lexer::Lexer;
use fsl::compiler::parser::Parser;

fn run_failang(source: &str) {
    let lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens().expect("Lexer failed");

    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser failed");

    let mut interpreter = Interpreter::new();
    interpreter.interpret(&program).expect("Runtime failed");
}

#[test]
fn test_collection_builtins_bilingual() {
    run_failang(
        r#"
        print len([1,2,3])
        print طول([1,2,3])

        print first([10,20,30])
        print أول([10,20,30])

        print last([10,20,30])
        print آخر([10,20,30])

        print contains([1,2,3],2)
        print يحتوي([1,2,3],5)
        "#,
    );
}

#[test]
fn test_text_and_conversion_builtins_bilingual() {
    run_failang(
        r#"
        print str(123)
        print نص(456)

        print number("789")
        print رقم("321")

        print replace("Failang","F","L")
        print استبدل("لغة فيصل","فيصل","FSL")

        print split("a,b,c",",")
        print تقسيم("x-y-z","-")

        print concat("Fail","ang")
        print دمج("F","SL")

        print trim("  hello  ")
        print قص_فراغات("  مرحبا  ")
        "#,
    );
}

#[test]
fn test_array_mutation_builtins_bilingual() {
    run_failang(
        r#"
        print reverse([1,2,3])
        print عكس([10,20,30])

        print is_empty([])
        print فارغ([])

        print pop([1,2,3])
        print احذف([4,5,6])
        "#,
    );
}
