extern crate pb_commit_message_lints;

use std::{env, process};

use clap::{crate_authors, crate_version, App, Arg};

use crate::PbCommitMessageError::PbCommitMessageLints;
use pb_commit_message_lints::{
    errors::PbCommitMessageLintsError,
    external::vcs::Git2,
    lints::{get_lint_configuration, lint, CommitMessage, LintCode, LintProblem},
};
use std::{
    convert::TryFrom,
    error::Error,
    fmt::{Display, Formatter},
    path::PathBuf,
};

const COMMIT_FILE_PATH_NAME: &str = "commit-file-path";

fn display_err_and_exit<T>(error: &PbCommitMessageError) -> T {
    eprintln!("{}", error);
    process::exit(1);
}

fn main() {
    let matches = app().get_matches();

    let commit_file_path = matches
        .value_of(COMMIT_FILE_PATH_NAME)
        .map(PathBuf::from)
        .expect("Expected file path name");

    let commit_message = CommitMessage::try_from(commit_file_path)
        .map_err(PbCommitMessageError::from)
        .unwrap_or_else(|err| display_err_and_exit(&err));

    let current_dir = env::current_dir()
        .map_err(|err| PbCommitMessageError::new_io("$PWD".into(), &err))
        .unwrap_or_else(|err| display_err_and_exit(&err));

    let git_config = Git2::try_from(current_dir)
        .map_err(PbCommitMessageError::from)
        .unwrap_or_else(|err| display_err_and_exit(&err));

    let output = format_lint_problems(
        &commit_message,
        lint(
            &commit_message,
            get_lint_configuration(&git_config)
                .map_err(PbCommitMessageError::from)
                .unwrap_or_else(|err| display_err_and_exit(&err)),
        ),
    );

    if let Some((message, exit_code)) = output {
        display_lint_err_and_exit(&message, exit_code)
    }
}

fn app() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name(COMMIT_FILE_PATH_NAME)
                .help(
                    "Path to a temporary file that contains the commit message written by the \
                     developer",
                )
                .index(1)
                .required(true),
        )
}

fn format_lint_problems(
    original_message: &CommitMessage,
    lint_problems: Vec<LintProblem>,
) -> Option<(String, LintCode)> {
    let (_, message_and_code) = lint_problems.into_iter().fold(
        (original_message, None),
        |(commit_message, output), item| {
            (
                commit_message,
                match output {
                    Some((existing_output, _)) => Some((
                        vec![existing_output, item.to_string()].join("\n\n"),
                        item.code(),
                    )),
                    None => Some((
                        vec![commit_message.to_string(), item.to_string()].join("\n\n---\n\n"),
                        item.code(),
                    )),
                },
            )
        },
    );
    message_and_code
}

fn display_lint_err_and_exit(commit_message: &str, exit_code: LintCode) {
    eprintln!("{}", commit_message);

    std::process::exit(exit_code as i32);
}

#[derive(Debug)]
enum PbCommitMessageError {
    PbCommitMessageLints(PbCommitMessageLintsError),
    Io(String, String),
}

impl PbCommitMessageError {
    fn new_io(location: String, error: &std::io::Error) -> PbCommitMessageError {
        PbCommitMessageError::Io(location, format!("{}", error))
    }
}

impl Display for PbCommitMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PbCommitMessageLints(error) => write!(f, "{}", error),
            PbCommitMessageError::Io(file_source, error) => write!(
                f,
                "Failed to read git config from `{}`:\n{}",
                file_source, error
            ),
        }
    }
}

impl From<PbCommitMessageLintsError> for PbCommitMessageError {
    fn from(err: PbCommitMessageLintsError) -> Self {
        PbCommitMessageLints(err)
    }
}

impl Error for PbCommitMessageError {}
