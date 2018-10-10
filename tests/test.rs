extern crate assert_cli;

#[test]
fn it_shows_help_for_no_args() {
    assert_cli::Assert::main_binary()
        .fails()
        .and()
        .stderr()
        .contains("USAGE")
        .unwrap();
}

#[test]
fn it_errors_if_cmd_not_found() {
    assert_cli::Assert::main_binary()
        .with_args(&["cmd", "test"])
        .fails()
        .and()
        .stderr()
        .contains("Command \"test\" not found in Cargo.toml")
        .unwrap();
}

#[test]
fn it_succeeds_when_command_is_found() {
    assert_cli::Assert::main_binary()
        .with_args(&["cmd", "pass"])
        .succeeds()
        .and()
        .stdout()
        .contains("> exit 0")
        .unwrap();
}

#[test]
fn it_returns_the_exit_code_of_the_command_when_it_fails() {
    assert_cli::Assert::main_binary()
        .with_args(&["cmd", "fail"])
        .fails_with(42)
        .and()
        .stdout()
        .contains("> exit 42")
        .unwrap();
}
