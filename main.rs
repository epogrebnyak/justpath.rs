use colored::*;
use std::env;
use std::path::PathBuf;

const DELIMITER: &str = if cfg!(windows) { ";" } else { ":" };

struct Directory {
    original: PathBuf,
    canonical: Option<PathBuf>,
}

impl Directory {
    fn new(original: &str) -> Self {
        let path = PathBuf::from(original);
        Directory {
            original: path.clone(),
            canonical: path.canonicalize().ok(),
        }
    }

    fn occurrences(&self, directories: &[Directory]) -> usize {
        directories
            .iter()
            .filter(|dir| dir.canonical == self.canonical)
            // was:
            //   .filter(|dir| dir.original == self.original)
            .count()
    }

    fn is_pointer(&self) -> bool {
        match &self.canonical {
            Some(canonical) => canonical != &self.original,
            None => false,
        }
    }
}

fn print_directories(directories: &[Directory]) {
    for (i, dir) in directories.iter().enumerate() {
        let count = dir.occurrences(directories);
        let number = format!("{:2}", i + 1).dimmed();
        let path_str = dir.original.display().to_string();

        if dir.original.exists() {
            // Start printing the line
            print!("{} {} ", number, path_str.green());

            // Print count message
            if count > 1 {
                print!("{}", format!("(duplicate ×{})", count).yellow().bold());
            } else {
                print!("{}", "(unique)".green().dimmed());
            }

            // Print canonical arrow on the same line if symlink
            if dir.is_pointer() {
                if let Some(canonical) = &dir.canonical {
                    print!(" {}", format!("→ {}", canonical.display()).dimmed());
                }
            }

            println!();
        } else {
            println!(
                "{} {} {}",
                number,
                path_str.red(),
                "(missing)".red().dimmed()
            );
        }
    }
}

fn main() {
    match env::var("PATH") {
        Ok(path) => {
            let directories: Vec<Directory> = path.split(DELIMITER).map(Directory::new).collect();
            print_directories(&directories);
        }
        Err(e) => println!("{} {}", "Error:".red().bold(), e),
    }
}
