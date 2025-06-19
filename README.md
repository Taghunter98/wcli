 # WCLI v1.0.0

 WCLI is a tool to help you run quick and easy commands to an EC2 instance.

 It allows you to run Linux commands remotely as root user, with added security of an `.env`
 file to hold the AWS credentials.

 You can also run predefined commands to help abstract repetitive processes including git, sql, and test, for
 Python unit tests as of version 1.0.0.

 ## Getting Started

 Create a `.env` file in the root directory of the project and structure it to contain the
 PEM file path, EC2 ip address and the root password.

 ```env
 PASS='password'
 EC2='ec2-user@ec2-xxxxxxxx.compute.amazonaws.com'
 PEM='/home/user/<your_file.pem>'
 ```
 Then build the project with `cargo run` and provided the AWS credentials are ok, WCLI will
 connect successfully.

 ### Global installation Linux/MacOS
 For a global installation run the following commands to be able to open with `wcli`.
 ```bash
 cargo build --release  # Build release version

 sudo cp target/release/wcli /usr/local/bin/ # Add to binary directory as an executable

 wcli   # Run and test 
 ```

 ## Examples and Usage
c
 WCLI is designed to feel like an interactive shell, you can run some predefined commands
 to help abstract from repetitve tasks.

 `cmd` opens a shell for direct commands to the EC2.

 _Note a limitation of the shell is that each command needs to be one line._

 ```plaintext
 [user@wcli ~]$ cmd
 Run 'help' for commands

 >>>  cd Directory && ls -l
 total 16
 drwxr-xr-x. 4 ec2-user ec2-user    33 Jun 18 13:47 Directory1
 drwxr-xr-x. 8 ec2-user ec2-user 16384 Jun 19 11:48 Directory2
 ```

 `git` opens a shell with a prompt for the repository.
 ```plaintext
 [user@wcli ~]$ git
 Repo path: Documents/repository
 Run 'help' for commands

 >>>  git status
 On branch dev
 Your branch is up to date with 'origin/dev'.
 ```

 `sql` opens a shell that attempts a connection to mariadb provided you are running MySQL.
 ```plaintext
 [user@wcli ~]$ sql
 Connected to mariadb in 191ns

 Database: mydb
 Run 'help' for commands

 >>>  SELECT name FROM Users;
 name
 Beth
 Josh
 ```

 `test` opens a shell that attempts to run Python unittests within a given directory. You
 also need to provide a virtual environment path from your repository.

 _Note future release will automatically create a .venv for you._

 ```plaintext
 [user@wcli ~]$ test
 Repo path: Documents/repository
 venv name: .venv
 Tests path: app/tests

 All tests passed in 6s
 ``````

 ## License

 Copyright (C) Josh Bassett. All rights reserved.

 Apache 2.0
