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
        .with_args(&["cmd", "notfound"])
        .fails()
        .and()
        .stderr()
        .contains("Command \"notfound\" not found in Cargo.toml")
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

#[test]
fn it_passes_extra_arguments_to_the_command() {
    assert_cli::Assert::main_binary()
        .with_args(&["cmd", "echo", "hello planet"])
        .succeeds()
        .and()
        .stdout()
        .contains("> echo hello planet")
        .unwrap();
}

#[test]
fn it_runs_the_pre_command() {
    assert_cli::Assert::main_binary()
        .with_args(&["cmd", "chain"])
        .succeeds()
        .and()
        .stdout()
        .contains("[prechain]")
        .unwrap();
}

#[test]
fn it_runs_the_post_command() {
    assert_cli::Assert::main_binary()
        .with_args(&["cmd", "chain"])
        .succeeds()
        .and()
        .stdout()
        .contains("[postchain]")
        .unwrap();
}

#[test]
fn it_labels_the_command_if_running_multiple() {
    assert_cli::Assert::main_binary()
        .with_args(&["cmd", "chain"])
        .succeeds()
        .and()
        .stdout()
        .contains("[chain]")
        .unwrap();
}

#[test]
fn it_stops_the_chain_if_a_command_fails() {
    assert_cli::Assert::main_binary()
        .with_args(&["cmd", "failchain"])
        .fails()
        .and()
        .stdout()
        .doesnt_contain("[chain]")
        .unwrap();
}
