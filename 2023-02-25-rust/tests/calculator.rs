use kata_carpaccio::run;

#[test]
fn test_run_returns_ok_when_no_issues() {
    let input = b"22 19.95";
    let result = run(&input[..]);
    assert_eq!(result, Ok("0".to_string()));
}

#[test]
fn test_run_returns_error_when_invalid_prices() {
    let input = b"Waka";
    let result = run(&input[..]);
    assert!(result.is_err());
}
