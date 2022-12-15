use std::{fs::File, io::Write, process::exit};

use clap::{arg, Command};

use crate::gh::{client::Client, data::User};
use crate::gh::data::Repository;

mod gh;

fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .disable_version_flag(true)
        .args(&[
            arg!(-v --version       "Get the version"),
            arg!(-d --debug         "Print debug information"),
            arg!(-s --save <path>   "Save response to files"),
            arg!(<token>            "The Personal Access Token for GitHub").required(true),
        ])
        .get_matches();

    if matches.get_flag("version") {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        exit(0);
    }

    let is_debug_enabled = matches.get_flag("debug");
    let save_file = matches.get_one::<String>("save");
    let token = matches.get_one::<String>("token")
        .expect("Error: token is required");

    let client = Client::new(token.as_str());
    let octocat = client.get_octocat().unwrap();
    println!("Octocat: {}", octocat);

    if let Ok(user) = client.get::<User>("user") {
        if is_debug_enabled {
            println!("{0:#?}", user);
        }
    }

    if let Ok(repos) = client.get::<Vec<Repository>>("user/repos") {
        if is_debug_enabled {
            println!("{0:#?}", repos);
        }
    }

    if let Ok(response) = client.get_user_orgs() {
        if is_debug_enabled {
            println!("{}", response);
        }
        if let Some(path) = save_file {
            let path = format!("{}/orgs.json", path);
            file_write_response(&path, response);
        }
    }

    if let Ok(response) = client.get_user_starred() {
        if is_debug_enabled {
            println!("{}", response);
        }
        if let Some(path) = save_file {
            let path = format!("{}/starred.json", path);
            file_write_response(&path, response);
        }
    }

    if let Ok(response) = client.get_user_watched() {
        if is_debug_enabled {
            println!("{}", response);
        }
        if let Some(path) = save_file {
            let path = format!("{}/watched.json", path);
            file_write_response(&path, response);
        }
    }
}

fn file_write_response(path: &str, response: String) {
    match File::create(path) {
        Ok(mut file) => {
            file.write_all(response.as_bytes()).expect("Error");
            file.flush().expect("Error");
        }
        Err(error) => println!("Error: {}", error),
    }
}
