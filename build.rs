//! Build script for WCLI.
//!
//! ## License
//!
//! Copyright (C) Josh Bassett. All rights reserved.
//!
//! Apache 2.0
//!

use std::{env, fs, path::Path};

/// Function first retrieves the env variables then writes a new config with CONST values
/// with the env variables. This means that wcli can be run globally.
/// 
fn main() {
  println!("cargo:rerun-if-changed=.env");

  dotenv::from_filename(".env").ok();

  let pass: String = env::var("PASS").expect("PASS must be set in .env");
  let pem: String  = env::var("PEM").expect("PEM must be set in .env");
  let ec2: String  = env::var("EC2").expect("EC2 must be set in .env");

  let dest = Path::new(&env::var("OUT_DIR").unwrap()).join("config.rs");
  fs::write(&dest, format!(
    r#"pub const PASS: &str = "{pass}";
    pub const PEM: &str  = "{pem}";
    pub const EC2: &str  = "{ec2}";
    "#
  )).unwrap();
}
