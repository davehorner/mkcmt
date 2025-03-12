use mkcmt::commit::CommitMessage;

#[test]
fn test_parse_breaking_change_with_proper_formatting() {
    // This input uses proper blank lines so that the parser splits the input into header, body, and footer.
    let input = r#"feat(ui): add new button

This commit adds a new button to the UI.

BREAKING CHANGE!: The button API has changed."#;

    let commit = CommitMessage::parse(input).expect("Failed to parse commit message");
    assert_eq!(commit.commit_type, "feat");
    assert_eq!(commit.scope, Some("ui".into()));
    assert_eq!(commit.description, "add new button");
    // With correct formatting, the footer is recognized and the breaking flag is set.
    assert!(commit.breaking, "Commit should be marked as breaking");
}

#[test]
fn test_parse_no_breaking_change_with_single_newlines() {
    // This input uses single newlines; the parser treats the entire input as a single section (header only)
    // so no footer is detected and breaking remains false.
    let input = "feat(ui): add new button\nThis commit adds a new button to the UI.\nBREAKING CHANGE!: The button API has changed.";

    let commit = CommitMessage::parse(input).expect("Failed to parse commit message");
    // Without the proper blank lines, no footer is parsed, so breaking should be false.
    assert!(!commit.breaking, "Commit should not be marked as breaking");
}
