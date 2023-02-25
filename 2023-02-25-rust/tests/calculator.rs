use kata_carpaccio::run;

#[test]
fn test_run_returns_ok_when_no_issues() {
    let input = b"0\n1";
    let result = run(&mut &input[..]);
    assert_eq!(result, Ok("0".to_string()));
}

#[test]
fn test_run_returns_error_when_invalid_prices() {
    let input = b"Waka";
    let result = run(&mut &input[..]);
    assert!(result.is_err());
}

#[test]
fn test_run_returns_error_when_invalid_units() {
    let input = b"10\nWaka";
    let result = run(&mut &input[..]);
    assert!(result.is_err());
}

#[test]
fn test_run_returns_the_gross_price_for_the_order() {
    let input = b"9 20\n1 2";
    let result = run(&mut &input[..]);
    assert_eq!(result, Ok(format!("{}", 49.0 * 1.21)));
}
