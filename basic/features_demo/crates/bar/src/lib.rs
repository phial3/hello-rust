pub fn print() -> String {
    let point = foo::Point::new(42);
    serde_json::to_string(&point.inner).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn print_test() {
        assert_eq!(print(), "[42,42]");
    }
}
