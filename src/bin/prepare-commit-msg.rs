extern crate git2;
extern crate regex;

use git2::Repository;
use regex::Regex;

use std::fs::{File, read_to_string};
use std::io::Write;
use std::process;
use std::env;

fn main() {
    let commit_filename = match env::args().nth(1) {
        Some(a) => a,
        None => {
            eprintln!("Commit file was not provided");
            process::exit(1);
        }
    };

    // The commit source will be filled with labels like 'merge', 'amend'
    // or 'message' to say how you got to this point.
    let commit_source = env::args().nth(2);

    let current_branch = get_current_branch().unwrap_or_else(|err| {
        eprintln!("Failed to find current branch. {}", err);
        process::exit(1);
    });

    if let Err(e) = prepend_branch_name(current_branch, commit_filename,
                                        commit_source) {
        eprintln!("Failed to prepend message. {}", e);
        process::exit(1);
    }
}

fn get_current_branch() -> Result<String, git2::Error> {
    let git_repo = Repository::discover("./")?;
    let head = git_repo.head()?;
    let head_name = head.shorthand();
    match head_name {
        Some(name) => Ok(name.to_string()),
        None => Err(git2::Error::from_str("No branch name found"))
    }
}

fn prepend_branch_name(branch_name: String,
                       commit_filename: String,
                       source: Option<String>) -> Result<(), std::io::Error> {
    if branch_name == "master".to_string() {
        println!("Branch is master. Not adding branch name to commit message.");
        return Ok(());
    }

    let current_message = read_to_string(&commit_filename)?;

    let re = Regex::new(r"^\s*(\w+-\d+)").unwrap();
    if !re.is_match(&current_message) {
        match re.captures(&branch_name) {
            None => eprintln!("Could not identify valid id in {}", branch_name),
            Some(m) => {
                let branch_id = m.get(1).map_or("", |m| m.as_str());
                write_file(&commit_filename, &current_message, branch_id, source == None)?;
            }
        }
    }
    Ok(())
}

fn write_file(commit_filename: &String,
              current_message: &String,
              branch_id: &str, newline: bool) -> Result<(), std::io::Error> {
    let mut commit_file = File::create(commit_filename)?;
    if newline {
        writeln!(commit_file, "{} --", branch_id)?;
    } else {
        write!(commit_file, "{} -- ", branch_id)?;
    }
    write!(commit_file, "{}", current_message)
}
