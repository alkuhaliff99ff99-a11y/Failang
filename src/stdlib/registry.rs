#[derive(Debug, Clone)]
pub struct StdFunction {
    pub english: &'static str,
    pub arabic: &'static str,
    pub module: &'static str,
}

pub fn functions() -> Vec<StdFunction> {
    vec![
        StdFunction {
            english: "abs",
            arabic: "مطلق",
            module: "math",
        },
        StdFunction {
            english: "max",
            arabic: "أكبر",
            module: "math",
        },
        StdFunction {
            english: "min",
            arabic: "أصغر",
            module: "math",
        },
        StdFunction {
            english: "length",
            arabic: "طول",
            module: "text",
        },
        StdFunction {
            english: "empty",
            arabic: "فارغ",
            module: "text",
        },
        StdFunction {
            english: "version",
            arabic: "إصدار",
            module: "system",
        },
    ]
}

pub fn find(name: &str) -> Option<StdFunction> {
    functions()
        .into_iter()
        .find(|f| f.english == name || f.arabic == name)
}
