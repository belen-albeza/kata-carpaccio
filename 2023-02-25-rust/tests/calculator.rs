use kata_carpaccio::run;

#[test]
fn test_run_returns_ok_when_no_issues() {
    let input = b"0";
    let result = run(&input[..]);
    assert_eq!(result, Ok("0".to_string()));
}

#[test]
fn test_run_returns_error_when_invalid_prices() {
    let input = b"Waka";
    let result = run(&input[..]);
    assert!(result.is_err());
}

#[test]
fn test_run_returns_the_net_price_for_the_order() {
    let input = b"9 20";
    let result = run(&input[..]);
    assert_eq!(result, Ok("29".to_string()));
}
