use clap::{arg, Command};
use std::{fs::File, io::Write, process::exit};

mod gh;

fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .disable_version_flag(true)
        .args(&[
            arg!(-v --version   "Gets the version"),
            arg!(-d --debug     "Prints debug information"),
            arg!(-s --save      "Saves response to files"),
            arg!(<token>        "The token required for querying GitHub").required(true),
        ])
        .get_matches();

    if matches.get_flag("version") {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        exit(0);
    }

    let is_debug_enabled = matches.get_flag("debug");
    let is_save_enabled = matches.get_flag("save");
    let token = matches.get_one::<String>("token")
        .expect("Error: token is required");

    let client = gh::GitHubClient::new(token.as_str());
    let octocat = client.get_octocat().unwrap();
    println!("Octocat: {}", octocat);

    // Get the user
    if let Ok(user) = client.get_user() {
        if is_debug_enabled {
            println!("{}", user);
        }
        if is_save_enabled {
            file_write_response("user.json", user);
        }
    }

    // Get the user repositories
    if let Ok(response) = client.get_user_repos() {
        if is_debug_enabled {
            println!("{}", response);
        }
        if is_save_enabled {
            file_write_response("repos.json", response);
        }
    }

    // Get the user organizations
    if let Ok(response) = client.get_user_orgs() {
        if is_debug_enabled {
            println!("{}", response);
        }
        if is_save_enabled {
            file_write_response("orgs.json", response);
        }
    }

    // Get the user starred repositories
    if let Ok(response) = client.get_user_starred() {
        if is_debug_enabled {
            println!("{}", response);
        }
        if is_save_enabled {
            file_write_response("starred.json", response);
        }
    }

    // Get the user starred repositories
    if let Ok(response) = client.get_user_subscriptions() {
        if is_debug_enabled {
            println!("{}", response);
        }
        if is_save_enabled {
            file_write_response("subscriptions.json", response);
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
