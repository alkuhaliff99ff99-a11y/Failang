pub fn abs(value: i64) -> i64 {
    if value < 0 {
        -value
    } else {
        value
    }
}

pub fn max(a: i64, b: i64) -> i64 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn min(a: i64, b: i64) -> i64 {
    if a < b {
        a
    } else {
        b
    }
}
