#[no_mangle]
pub extern fn add(a: i64, b: i64) -> i64 {
    a + b
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(2,2), 4);
    }
}
