use kata_carpaccio::run;

#[test]
fn test_runs_returns_ok_when_no_issues() {
    let input = b"Waka";
    let result = run(&input[..]);
    assert_eq!(result, Ok("Waka".to_string()));
}
