use skan::scanner::Scanner;

#[test]
pub fn test_string_with_zero_length() {
    let scan = Scanner::from_str("");
    assert_eq!(scan.has_next(), false);
}

#[test]
pub fn test_string_with_single_length() {
    let scan = Scanner::from_str("1");
    assert_eq!(scan.has_next(), true);
}

#[test]
pub fn test_next_implmentation_function() {
    let mut scan = Scanner::from_str("java");
    assert_eq!(scan.next(), Some("java".as_bytes()[0]));
}

#[test]
pub fn test_number_from_stdin() {
    let mut scan = Scanner::from_str("67 -89");
    assert_eq!(
        [
            scan.next_number::<i32>().unwrap(),
            scan.next_number::<i32>().unwrap()
        ],
        [67, -89]
    );
}
