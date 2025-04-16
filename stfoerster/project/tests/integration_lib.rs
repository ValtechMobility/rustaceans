use prj::parse_input;

#[test]
fn test_parseOfInput() {
    let input = r#"say "Hallo Welt""#;
    let (cmd, args, opts) = parse_input(input);

    assert_eq!(cmd, "say");
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], "Hallo Welt");

}

#[test]
fn test_shouldParseOnlyTheCmd() {
    let input = r#"say"#;
    let (cmd, args, opts) = parse_input(input);

    assert_eq!(cmd, "say");

}