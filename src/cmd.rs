//! This module provides a high level API to run Linux commands on an EC2.
//!
//! ## License
//!
//! Copyright (C) Josh Bassett. All rights reserved.
//!
//! Apache 2.0
//!

use std::{
    io::{self, Write},
    process::{Command, Output},
};

use colored::Colorize;
use indicatif::ProgressBar;

/// Runs a Linux command remotely on an EC2.
///
/// Function checks if sudo is the first argument then runs the command as root.
///
/// # Errors
///
/// - Returns [`expect`](core::result::Result<Error>) if stdout fails to flush.
/// - Returns [`expect`](core::result::Result<Error>) if input is unreadable.
/// - Returns [`stderr`](std::process::Output) if the returned command is an error.
///
/// # Examples
/// ```rust
/// let password: String = var("PASS").expect("Password is needed to run sudo commands");
/// cmd(&password)
/// ```
///
pub fn cmd(password: &str) {
    println!("Run 'help' for commands\n");
    loop {
        let bash_cmd = input();

        let mut parts: std::str::SplitN<'_, char> = bash_cmd.splitn(2, ' ');
        let first: &str = parts.next().unwrap_or("");

        match first.trim() {
            "sudo" => helpers::print_cmd(&run_cmd_sudo(&bash_cmd, password)),
            "install" => install(password),
            "remove" => remove(password),
            "clear" => helpers::clear(),
            "help" => cmd_help(),
            "exit" => break,
            _ => helpers::print_cmd(&run_cmd(bash_cmd.trim())),
        }
    }
}

/// Returns user input.
///
/// # Errors
///
/// - Returns [`expect`](core::result::Result<Error>) if stdout fails to flush.
/// - Returns [`expect`](core::result::Result<Error>) if input is unreadable.
///
fn input() -> String {
    print!("{} ", ">>> ".purple());
    io::stdout().flush().expect("failed to flush stdout");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    input
}

/// Returns user input with a prompt.
///
/// # Errors
///
/// - Returns [`expect`](core::result::Result) if stdout fails to flush.
/// - Returns [`expect`](core::result::Result<Error>) if input is unreadable.
///
fn msg_input(msg: &str) -> String {
    print!("{msg}: ");
    io::stdout().flush().expect("failed to flush stdout");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    input
}

/// Returns Output of bash command from EC2.
///
/// # Errors
///
/// - Returns [`expect`](Result<Error>) if input is unreadable.
///
fn run_cmd(bash_cmd: &str) -> Output {
    let bar: ProgressBar = helpers::new_bar();
    bar.enable_steady_tick(std::time::Duration::from_millis(80));

    let output: Output = Command::new("bash")
        .arg("-c")
        .arg(connect::ssh(bash_cmd))
        .output()
        .expect("failed to execute remote command");

    bar.finish_and_clear();

    output
}

/// Retruns Output of sudo bash command from EC3
///
/// # Errors
///
/// - Returns [`expect`](Result<Error>) if bash_cmd has bad inputs or an incorrect password.
///
fn run_cmd_sudo(sudo_cmd: &str, password: &str) -> Output {
    let bash_cmd: String = format!("echo {password} | {sudo_cmd}");

    run_cmd(&bash_cmd)
}

/// Provides an abstraction for installing packages with yum.
///
/// # Errors
///
/// - Returns [`expect`](Result<Error>) if bash_cmd has bad inputs or an incorrect password.
/// - Returns [`stderr`](std::process::Output) if the returned command is an error.
///
fn install(password: &str) {
    let package: String = msg_input("Package");

    let bash_cmd: String = format!("echo {password} | sudo yum install -y {package}");

    helpers::print_cmd(&run_cmd(&bash_cmd));
}

/// Provides an abstraction for removing packages with yum.
///
/// # Errors
///
/// - Returns [`expect`](Result<Error>) if bash_cmd has bad inputs or an incorrect password.
/// - Returns [`stderr`](std::process::Output) if the returned command is an error.
///
fn remove(password: &str) {
    let package = msg_input("Package");
    let bash_cmd: String = format!("echo {password} | sudo yum remove -y {package}");

    helpers::print_cmd(&run_cmd(&bash_cmd));
}

/// Displays a help message.
///
fn cmd_help() {
    println!("\nCOMMANDS");
    println!("'any'         -> run a Linux cmd, ensure syntax is correct");
    println!("'install'     -> install a package");
    println!("'remove'      -> uninstall a package");
    println!("'clear'       -> clears the terminal");
    println!("'exit'        -> exit cmd");
}

pub mod connect {
    //! This module provides an API for ssh login to an EC2.
    //!
    //! ## License
    //!
    //! Copyright (C) Josh Bassett. All rights reserved.
    //!
    //! Apache 2.0
    //!

    // Rust analyser genrates warning as env files are only known at compile time
    include!(concat!(env!("OUT_DIR"), "/config.rs"));

    use std::time::SystemTime;

    use colored::Colorize;

    use crate::cmd::run_cmd;

    /// Returns an ssh connection string.
    ///
    /// # Errors
    ///
    /// - Returns [`expect`](Result<Error>) if an env variable isn't reachable.
    ///
    pub fn ssh(bash_cmd: &str) -> String {
        let pem: String = PEM.to_string();
        let ec2: String = EC2.to_string();

        format!("ssh -i {} {} '{}'", pem, ec2, bash_cmd)
    }

    /// Tests connection to EC2 using ssh.
    ///
    /// # Errors
    ///
    /// - Returns [`expect`](Result<Error>) if bash_cmd has bad inputs or an incorrect password.
    /// - Returns [`except`](std::process::Output) if current time is unreachable.
    /// - Panics [`panic`](std::macros) if connection is unsuccessful.
    ///
    pub fn test_connection() {
        let output: std::process::Output = run_cmd("echo test");
        let now = SystemTime::now().elapsed().expect("unable to get time");
        if output.status.success() {
            println!(
                "{} to EC2 on {} in {:?}\n",
                "Connected".green(),
                chrono::offset::Local::now().format("%a %b %e at %T"),
                now
            )
        } else {
            panic!("unable to connect to EC2")
        }
    }
}

pub mod git {
    //! This module provides an API for running git commands.
    //!
    //! ## License
    //!
    //! Copyright (C) Josh Bassett. All rights reserved.
    //!
    //! Apache 2.0
    //!

    use crate::cmd::{
        helpers::{self, clear},
        input, msg_input, run_cmd,
    };

    /// Provides an API for running git commands.
    ///
    /// # Errors
    ///
    /// - Returns [`expect`](Result<Error>) if bash_cmd has bad inputs or an incorrect password.
    ///
    /// # Examples
    /// ```rust
    /// run_git();
    /// ```
    /// Output
    /// ```plaintext
    /// [user@wcli ~]$ git   
    /// Repo path: directory/repository
    /// >>>  git status
    /// On branch dev
    /// ...
    /// ```
    pub fn run_git() {
        let directory: String = msg_input("Repo path");
        println!("Run 'help' for commands\n");

        loop {
            let user_cmd: String = input();

            match user_cmd.trim() {
                "exit" => break,
                "change" => run_git(),
                "clear" => clear(),
                "help" => git_help(),
                _ => git_cmd(directory.as_str().trim(), user_cmd.as_str()),
            }
        }
    }

    /// Runs a git command on the EC2 instance.
    ///
    /// # Errors
    ///
    /// - Returns [`expect`](Result<Error>) if bash_cmd has bad inputs or an incorrect password.
    /// - Returns [`stderr`](std::process::Output) if the returned command is an error.
    ///
    /// # Examples
    /// ```rust
    /// run_git();
    /// ```
    /// Output
    /// ```plaintext
    /// [user@wcli ~]$ git   
    /// Repo path: directory/repository
    /// >>>  git status
    /// On branch dev
    /// ...
    /// ```
    fn git_cmd(directory: &str, user_cmd: &str) {
        let git_cmd: String = format!("cd {directory} && {user_cmd}");

        let output = run_cmd(&git_cmd);

        helpers::print_cmd(&output);
    }

    /// Prints out a help message.
    ///
    fn git_help() {
        println!("\nCOMMANDS");
        println!("'any'     -> run a git command, ensure syntax is correct");
        println!("'change'  -> change git directory");
        println!("'clear'   -> clears the terminal");
        println!("'exit'    -> exit git");
    }
}

pub mod sql {
    //! This module provides an API for running sql commands.
    //!
    //! ## License
    //!
    //! Copyright (C) Josh Bassett. All rights reserved.
    //!
    //! Apache 2.0
    //!

    use std::time::SystemTime;

    use colored::Colorize;

    use crate::cmd::{
        helpers::{self, clear},
        input, msg_input, run_cmd,
    };

    /// Provides an API for running sql commands.
    ///
    /// # Errors
    ///
    /// - Returns [`expect`](Result<Error>) if bash_cmd has bad inputs or an incorrect password.
    /// - Panics [`panic`](std::macros) if mariadb connection is unsuccessful.
    /// - Returns [`stderr`](std::process::Output) if the returned command is an error.
    ///
    /// # Examples
    /// ```rust
    /// run_sql(&password);
    /// ```
    /// Select emails from table
    /// ```plaintext
    /// >>> SELECT email FROM <Table>;
    /// ```
    /// Describe a table
    /// ```plaintext
    /// >>> DESCRIBE <Table>;
    /// ```
    /// Delete a table
    /// ```plaintext
    /// >>> DROP Table <Table>;
    /// ```
    pub fn run_sql(password: &str) {
        test_sql_connection(password);

        let database: String = msg_input("Database");
        println!("Run 'help' for commands\n");

        loop {
            let query: String = input();

            match query.trim() {
                "exit" => break,
                "database" => {
                    println!("In database: {}", &database);
                }
                "change" => run_sql(password),
                "clear" => clear(),
                "help" => sql_help(),
                _ => sql_query(password, database.as_str().trim(), query.as_str().trim()),
            }
        }
    }

    /// Runs a sql query on the EC2.
    ///
    /// ## Errors
    ///
    /// - Returns [`expect`](Result<Error>) if bash_cmd has bad inputs or an incorrect password.
    /// - Returns [`stderr`](std::process::Output) if the returned command is an error.
    fn sql_query(password: &str, database: &str, query: &str) {
        let sql_cmd: String =
            format!("echo {password} | sudo -S mariadb -u root -p -e \"USE {database}; {query}\"");

        let output = run_cmd(&sql_cmd);
        helpers::print_cmd(&output);
    }

    /// Tests the connection to mariadb.
    ///
    /// # Errors
    ///
    /// - Panics [`panic`](std::macros) if mariadb connection is unsuccessful.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let password: &str = "password";
    /// let res: bool = test_sql_connection(&password);
    /// ```
    ///
    fn test_sql_connection(password: &str) {
        let sql_cmd: String = format!("echo {password} | sudo -S mariadb -u root -p");

        let output: std::process::Output = run_cmd(&sql_cmd);
        let now = SystemTime::now().elapsed().expect("unable to get time");

        if output.status.success() {
            println!("{} to mariadb in {:?}\n", "Connected".green(), now)
        } else {
            panic!("unable to connect to mariadb")
        }
    }

    /// Prints out a help message.
    ///
    fn sql_help() {
        println!("\nCOMMANDS");
        println!("'any'         -> run a sql query, ensure syntax is correct");
        println!("'change db'   -> show current database");
        println!("'clear'       -> clears the terminal");
        println!("'exit'        -> exit sql");
    }
}

pub mod test {
    //! This module provides an API for running Python unittests.
    //!
    //! ## License
    //!
    //! Copyright (C) Josh Bassett. All rights reserved.
    //!
    //! Apache 2.0
    //!
    
    use std::time::Instant;

    use crate::cmd::{msg_input, run_cmd};

    /// Provides an API for running Python unittests.
    /// 
    /// Function requires user to input the directory, venv and test directory.
    /// 
    /// TODO - automatic venv creation.
    ///
    /// # Errors
    ///
    /// - Returns [`expect`](Result<Error>) if any input is unreadable.
    /// - Returns [`stderr`](std::process::Output) if the returned command is an error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// run_unittests();
    /// ```
    /// Output
    /// ```plaintext
    /// [user@wcli ~]$ test
    /// Repo path: Directory/repository
    /// venv name: .venv
    /// Tests path: app/tests
    /// 
    /// All tests passed in 6s
    /// ```
    /// 
    pub fn run_unittests() {
        let directory: String = msg_input("Repo path");
        let venv: String = msg_input("venv name");
        let tests: String = msg_input("Tests path");

        let bash_cmd = format!(
            "cd {} && source {}/bin/activate && python3 -m unittest discover {}",
            directory.as_str().trim(),
            venv.as_str().trim(),
            tests.as_str().trim()
        );

        test_cmd(&bash_cmd);
    }

    /// Runs a Linux command to execute Python unittests in a module.
    /// 
    /// Function times the runtime of the tests to get a very acurate runtime measurement off by
    /// 1ns. Failed test results will appear in the terminal.
    ///
    /// # Errors
    /// 
    /// - Returns [`stderr`](std::process::Output) if the returned command is an error.
    /// 
    fn test_cmd(bash_cmd: &str) {
        let now: Instant = Instant::now();

        let output: std::process::Output = run_cmd(bash_cmd);

        let elapsed: u64 = now.elapsed().as_secs();

        if output.status.success() {
            println!("\nAll tests passed in {}s", elapsed);
            println!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            println!("\nTests failed after {}s", elapsed);
            println!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }
}

pub mod helpers {
    //! This module provides helper functions.
    //!
    //! ## License
    //!
    //! Copyright (C) Josh Bassett. All rights reserved.
    //!
    //! Apache 2.0
    //!

    use std::process::{Command, Output};

    use indicatif::{ProgressBar, ProgressStyle};

    /// Prints `stdout` or `stderr` to the terminal.
    /// 
    /// /// # Errors
    /// 
    /// - Returns [`stderr`](std::process::Output) if the returned command is an error.
    /// 
    pub fn print_cmd(output: &Output) {
        if output.status.success() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            println!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }

    /// Clears the terminal with the `clear` command.
    /// 
    /// /// # Errors
    /// 
    /// - Returns [`stderr`](std::process::Output) if the returned command is an error.
    /// 
    pub fn clear() {
        let mut cmd: Command = Command::new("clear");
        let output: Output = cmd.output().expect("command was not executed");

        print_cmd(&output);
    }

    /// Creates a new loading spinner with incrementing bullet points.
    /// 
    pub fn new_bar() -> ProgressBar {
        let bar: ProgressBar = ProgressBar::new_spinner();

        let binding: Vec<String> = (1..10)
            .map(|n: usize| ".".repeat(n))
            .collect::<Vec<String>>();

        let bullets: Vec<&str> = binding.iter().map(|s: &String| s as &str).collect();

        bar.set_style(ProgressStyle::default_spinner().tick_strings(&bullets));

        bar
    }

    /// Prints help message.
    /// 
    pub fn help() {
        println!("\nCOMMANDS");
        println!("'cmd'     -> run a Linux command");
        println!("'test'    -> run Python unit tests");
        println!("'git'     -> run a git command in a repository");
        println!("'sql'     -> run a sql query, run 'help' for assistance");
        println!("'clear'   -> clear the terminal");
        println!("'exit'    -> exit wcli");
    }

    /// Retrieves the system username with Linux `whoami` command.
    /// 
    /// # Errors
    /// 
    /// - Returns [`stderr`](std::process::Output) if the returned command is an error.
    /// 
    pub fn check_name() -> String {
        let output = Command::new("bash")
            .arg("-c")
            .arg("whoami")
            .output()
            .expect("failed to execute command");

        if output.status.success() {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        } else {
            "user".to_string()
        }
    }

    /// Capitalises user's name.
    /// 
    pub fn capitalise(user: &str) -> String {
        let mut chars: Vec<char> = user.chars().collect();
        chars[0].make_ascii_uppercase();

        String::from_iter(chars)
    }
}
