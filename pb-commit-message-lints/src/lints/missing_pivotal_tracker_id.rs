use regex::Regex;

use crate::lints::{CommitMessage, LintCode, LintProblem};

const REGEX_PIVOTAL_TRACKER_ID: &str =
    r"(?i)\[(((finish|fix)(ed|es)?|complete[ds]?|deliver(s|ed)?) )?#\d+([, ]#\d+)*]";

fn has_missing_pivotal_tracker_id(commit_message: &CommitMessage) -> bool {
    has_no_pivotal_tracker_id(commit_message)
}

fn has_no_pivotal_tracker_id(text: &CommitMessage) -> bool {
    let re = Regex::new(REGEX_PIVOTAL_TRACKER_ID).unwrap();
    !text.matches_pattern(&re)
}

pub(crate) fn lint_missing_pivotal_tracker_id(
    commit_message: &CommitMessage,
) -> Option<LintProblem> {
    if has_missing_pivotal_tracker_id(commit_message) {
        Some(LintProblem::new(
            PIVOTAL_TRACKER_HELP.into(),
            LintCode::PivotalTrackerIdMissing,
        ))
    } else {
        None
    }
}

const PIVOTAL_TRACKER_HELP: &str = r#"
Your commit is missing a Pivotal Tracker Id

You can fix this by adding the Id in one of the styles below to the commit message
[Delivers #12345678]
[fixes #12345678]
[finishes #12345678]
[#12345884 #12345678]
[#12345884,#12345678]
[#12345678],[#12345884]
This will address [#12345884]
"#;

#[cfg(test)]
mod tests_has_missing_pivotal_tracker_id {
    #![allow(clippy::wildcard_imports)]

    use pretty_assertions::assert_eq;

    use crate::lints::CommitMessage;

    use super::*;

    #[test]
    fn with_id() {
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[#12345678]
"#,
            &None,
        );
    }

    fn test_has_missing_pivotal_tracker_id(message: &str, expected: &Option<LintProblem>) {
        let actual = &lint_missing_pivotal_tracker_id(&CommitMessage::new(message.into()));
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }

    #[test]
    fn multiple_ids() {
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[#12345678,#87654321]
"#,
            &None,
        );
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[#12345678,#87654321,#11223344]
"#,
            &None,
        );
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[#12345678 #87654321 #11223344]
"#,
            &None,
        );
    }

    #[test]
    fn id_with_fixed_state_change() {
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[fix #12345678]
"#,
            &None,
        );
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[FIX #12345678]
"#,
            &None,
        );
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[Fix #12345678]
"#,
            &None,
        );
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[fixed #12345678]
"#,
            &None,
        );
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[fixes #12345678]
"#,
            &None,
        );
    }

    #[test]
    fn id_with_complete_state_change() {
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[complete #12345678]
"#,
            &None,
        );

        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[completed #12345678]
"#,
            &None,
        );

        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[Completed #12345678]
"#,
            &None,
        );

        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[completes #12345678]
"#,
            &None,
        );
    }

    #[test]
    fn id_with_finished_state_change() {
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[finish #12345678]
"#,
            &None,
        );

        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[finished #12345678]
"#,
            &None,
        );
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[finishes #12345678]
"#,
            &None,
        );
    }

    #[test]
    fn id_with_delivered_state_change() {
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[deliver #12345678]
"#,
            &None,
        );

        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[delivered #12345678]
"#,
            &None,
        );
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[delivers #12345678]
"#,
            &None,
        );
    }

    #[test]
    fn id_with_state_change_and_multiple_ids() {
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[fix #12345678 #12345678]
"#,
            &None,
        );
    }

    #[test]
    fn id_with_prefixed_text() {
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

Finally [fix #12345678 #12345678]
"#,
            &None,
        );
    }

    #[test]
    fn invalid_state_change() {
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[fake #12345678]
"#,
            &Some(LintProblem::new(
                "\nYour commit is missing a Pivotal Tracker Id\n\nYou can fix this by adding the \
                 Id in one of the styles below to the commit message\n[Delivers \
                 #12345678]\n[fixes #12345678]\n[finishes #12345678]\n[#12345884 \
                 #12345678]\n[#12345884,#12345678]\n[#12345678],[#12345884]\nThis will address \
                 [#12345884]\n"
                    .into(),
                LintCode::PivotalTrackerIdMissing,
            )),
        );
    }

    #[test]
    fn missing_id_with_square_brackets() {
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit
"#,
            &Some(LintProblem::new(
                "\nYour commit is missing a Pivotal Tracker Id\n\nYou can fix this by adding the \
                 Id in one of the styles below to the commit message\n[Delivers \
                 #12345678]\n[fixes #12345678]\n[finishes #12345678]\n[#12345884 \
                 #12345678]\n[#12345884,#12345678]\n[#12345678],[#12345884]\nThis will address \
                 [#12345884]\n"
                    .into(),
                LintCode::PivotalTrackerIdMissing,
            )),
        );

        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[#]
"#,
            &Some(LintProblem::new(
                "\nYour commit is missing a Pivotal Tracker Id\n\nYou can fix this by adding the \
                 Id in one of the styles below to the commit message\n[Delivers \
                 #12345678]\n[fixes #12345678]\n[finishes #12345678]\n[#12345884 \
                 #12345678]\n[#12345884,#12345678]\n[#12345678],[#12345884]\nThis will address \
                 [#12345884]\n"
                    .into(),
                LintCode::PivotalTrackerIdMissing,
            )),
        );
    }
}
