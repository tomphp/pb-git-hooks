use std::{io::Write, process::Command};

use tempfile::NamedTempFile;

use pb_hook_test_helper::{assert_output, setup_working_dir};

#[test]
fn valid_commit() {
    let input = r#"An example commit

This is an example commit without the JIRA Issue Key

JRA-123
"#;
    let working_dir = setup_working_dir();
    Command::new("git")
        .current_dir(&working_dir)
        .arg("config")
        .arg("--local")
        .arg("pb.lint.jira-issue-key-missing")
        .arg("true")
        .output()
        .expect("failed to execute process");

    let mut commit_path = NamedTempFile::new().unwrap();
    write!(commit_path, "{}", input).unwrap();

    let output = pb_hook_test_helper::run_hook(
        &working_dir,
        "pb-commit-msg",
        vec![commit_path.path().to_str().unwrap()],
    );

    assert_output(&output, "", "", true)
}

#[test]
fn explicitly_enabled() {
    let input = r#"An example commit

This is an example commit without the JIRA Issue Key
"#;
    let working_dir = setup_working_dir();
    Command::new("git")
        .current_dir(&working_dir)
        .arg("config")
        .arg("--local")
        .arg("pb.lint.jira-issue-key-missing")
        .arg("true")
        .output()
        .expect("failed to execute process");

    let mut commit_path = NamedTempFile::new().unwrap();
    write!(commit_path, "{}", input).unwrap();

    let output = pb_hook_test_helper::run_hook(
        &working_dir,
        "pb-commit-msg",
        vec![commit_path.path().to_str().unwrap()],
    );

    let expected_stderr = r#"An example commit

This is an example commit without the JIRA Issue Key


---


Your commit is missing a JIRA Issue Key

You can fix this by adding a key like `JRA-123` to the commit message

"#;

    assert_output(&output, "", expected_stderr, false)
}

#[test]
fn disabled() {
    let input = r#"An example commit

This is an example commit without the jira issue key
"#;
    let working_dir = setup_working_dir();
    Command::new("git")
        .current_dir(&working_dir)
        .arg("config")
        .arg("--local")
        .arg("pb.lint.jira-issue-key-missing")
        .arg("false")
        .output()
        .expect("failed to execute process");

    let mut commit_path = NamedTempFile::new().unwrap();
    write!(commit_path, "{}", input).unwrap();

    let output = pb_hook_test_helper::run_hook(
        &working_dir,
        "pb-commit-msg",
        vec![commit_path.path().to_str().unwrap()],
    );
    assert_output(&output, "", r#""#, true)
}
