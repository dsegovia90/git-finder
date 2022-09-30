use clap::Parser;
use std::{collections::VecDeque, fs, path::PathBuf};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory
    #[arg(default_value_t = String::from("/"))]
    dir: String,
    // ///Number of time to greet
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
}

fn main() {
    let args = Args::parse();

    let dir_list = find_all_git_dirs(args.dir);

    for dir in dir_list.unwrap() {
        if let Ok(p) = dir.canonicalize() {
            println!("{:?}", p.as_path())
        }
    }
}

fn find_all_git_dirs(initial_path: String) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut git_paths = Vec::new();

    let (mut dirs, initial_path_is_git_dir) =
        push_back_all_dirs_to_stack(PathBuf::from(&initial_path), None)?;

    if initial_path_is_git_dir {
        git_paths.push(PathBuf::from(initial_path));
    }

    while !dirs.is_empty() {
        let dir_option = dirs.pop_front();

        if let Some(dir) = dir_option {
            let path_dir = dir.to_owned();
            let (modded_dirs, path_is_git_dir) = push_back_all_dirs_to_stack(dir, Some(dirs))?;
            dirs = modded_dirs;

            if path_is_git_dir {
                git_paths.push(path_dir);
            };
        }
    }

    Ok(git_paths)
}

fn push_back_all_dirs_to_stack(
    initial_path: PathBuf,
    stack: Option<VecDeque<PathBuf>>,
) -> Result<(VecDeque<PathBuf>, bool), std::io::Error> {
    match fs::read_dir(&initial_path) {
        Ok(paths) => {
            let mut is_git_dir = false;

            let mut stack = match stack {
                Some(s) => s,
                None => VecDeque::new(),
            };

            for dir_entry_result in paths {
                match dir_entry_result {
                    Ok(dir_entry) => {
                        let path = dir_entry.path();

                        if path.ends_with(".git") {
                            is_git_dir = true;
                        }

                        if path.is_dir() {
                            stack.push_back(path);
                        }
                    }
                    Err(e) => {
                        print!("Skipping: \n{}", e);
                    }
                };
            }
            Ok((stack, is_git_dir))
        }
        Err(e) => {
            println!("Skipping dir {} because: \n{}\n", initial_path.display(), e);
            Ok((stack.unwrap_or_default(), false))
        }
    }
}
