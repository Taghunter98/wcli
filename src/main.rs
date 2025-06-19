//! WCLIv1.0.0
//!
//! WCLI is a tool to help you run quick and easy commands to an EC2 instance.
//!
//! It allows you to run Linux commands remotely as root user, with added security of an .env
//! file to hold the AWS credentials.
//!
//! You can also run predefined commands to help abstract including git, sql, and test, for
//! Python unit tests as of version 1.0.0.
//!
//! ## Getting Started
//!
//! Create a `.env` file in the root directory of the project and structure it to contain the
//! PEM file path, EC2 ip address and the root password.
//!
//! ```env
//! PASS='password'
//! EC2='ec2-user@ec2-xxxxxxxx.compute.amazonaws.com'
//! PEM='/home/user/<your_file.pem>'
//! ```
//! Then build the project with `cargo run` and provided the AWS credentials are ok, WCLI will
//! connect successfully.
//!
//! ## Examples and Usage
//!
//! WCLI is designed to feel like an interactive shell, you can run some predefined commands
//! to help abstract from repetitve tasks.
//!
//! `cmd` opens a shell for direct commands to the EC2.
//!
//! _Note a limitation of the shell is that each command needs to be one line._
//!
//! ```plaintext
//! [user@wcli ~]$ cmd
//! Run 'help' for commands
//!
//! >>>  cd Directory && ls -l
//! total 16
//! drwxr-xr-x. 4 ec2-user ec2-user    33 Jun 18 13:47 Directory1
//! drwxr-xr-x. 8 ec2-user ec2-user 16384 Jun 19 11:48 Directory2
//! ```
//!
//! `git` opens a shell with a prompt for the repository.
//! ```plaintext
//! [user@wcli ~]$ git
//! Repo path: Documents/repository
//! Run 'help' for commands
//!
//! >>>  git status
//! On branch dev
//! Your branch is up to date with 'origin/dev'.
//! ```
//!
//! `sql` opens a shell that attempts a connection to mariadb provided you are running MySQL.
//! ```plaintext
//! [user@wcli ~]$ sql
//! Connected to mariadb in 191ns
//!
//! Database: mydb
//! Run 'help' for commands
//!
//! >>>  SELECT name FROM Users;
//! name
//! Beth
//! Josh
//! ```
//!
//! `test` opens a shell that attempts to run Python unittests within a given directory. You
//! also need to provide a virtual environment path from your repository.
//!
//! _Note future release will automatically create a .venv for you._
//!
//! ```plaintext
//! [user@wcli ~]$ test
//! Repo path: Documents/repository
//! venv name: .venv
//! Tests path: app/tests
//!
//! All tests passed in 6s
//! ``````
//!
//! ## License
//!
//! Copyright (C) Josh Bassett. All rights reserved.
//!
//! Apache 2.0
//!

// Rust analyser genrates warning as env files are only known at compile time
include!(concat!(env!("OUT_DIR"), "/config.rs"));

use colored::Colorize;
use dotenv::dotenv;
use std::{
    io::{self, Write},
    process,
};

mod cmd;

fn main() {
    dotenv().ok();

    let password: String = PASS.to_string();
    let title: &'static str = "WCLI 2025";
    let version: &'static str = "Version 1.0.0";
    let website: &'static str = "https://github.com/Taghunter98/wcli.git";

    let logo: colored::ColoredString = format!(
        r#"
                 _  _   
                | |(_)  
 __      __ ___ | | _   {}
 \ \ /\ / // __|| || |  {}
  \ V  V /| (__ | || |  
   \_/\_/  \___||_||_|  {}

"#,
        title, version, website
    )
    .bold();

    println!("{logo}");
    let user: String = cmd::helpers::check_name();
    println!(
        "Welcome to WCLI {}! Run 'help' for commands\n",
        cmd::helpers::capitalise(&user)
    );

    cmd::connect::test_connection();

    main_loop(password, user);
}

/// Main loop, takes prompt from user and matches it with args for running Linux commands.
///
/// # Examples
///
/// ```rust
/// main_loop(password);
/// ```
/// Running a command
/// ```plaintext
/// [user@wcli ~]$ cmd
/// >>> ls
/// Running a sudo command
/// ```plaintext
/// [user@wcli ~]$ cmd
/// >>> sudo yum install docker
/// ```
/// Running a git command
/// ```plaintext
/// >>> git
/// [user@wcli ~]$ git
/// Enter repo path >>> Documents/repository
/// >>> git pull
/// ```
fn main_loop(password: String, user: String) {
    loop {
        print!("[{}@wcli ~]$ ", user);

        io::stdout().flush().expect("failed to flush stdout");

        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let prompt: &str = input.trim();

        match prompt {
            "cmd" => cmd::cmd(&password),
            "git" => cmd::git::run_git(),
            "sql" => cmd::sql::run_sql(password.as_str()),
            "test" => cmd::test::run_unittests(),
            "clear" => cmd::helpers::clear(),
            "help" => cmd::helpers::help(),
            "exit" => process::exit(1),
            _ => println!("invalid command, run 'help' for commands"),
        }
    }
}
