#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::error;
use std::error::Error as E;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct Error {
    message: String,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error { message: e.description().to_string() }
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error { message: e.description().to_string() }
    }
}

#[derive(Deserialize, Debug)]
struct Cargo {
    package: Package,
    bin: Vec<Bin>,
}

#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Bin {
    name: String,
    path: String,
}

fn read_cargo_toml(filename: &str) -> Result<String, Error> {
    fn read(mut f: File) -> String {
        let mut s = String::new();
        f.read_to_string(&mut s);
        s
    }
    File::open(filename).map(read).or_else(|e| Err(Error::from(e)))
}

fn parse(source: String) -> Result<Cargo, Error> {
    toml::from_str(&source).or_else(|e| Err(Error::from(e)))
}

fn show(cargo: Cargo) {
    for bin in cargo.bin {
        println!("{}", bin.name);
    }
}

fn main() {
    if let Ok(cargo) = read_cargo_toml("Cargo.toml").and_then(parse) {
        show(cargo);
    }
}
