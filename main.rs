use colored::*;
use std::env;
use std::path::PathBuf;

const DELIMITER: &str = if cfg!(windows) { ";" } else { ":" };

struct Directory {
    original: PathBuf,
    canonical: Option<PathBuf>,
}

struct DirectoryWithCounts {
    original: PathBuf,
    canonical: Option<PathBuf>,
    duplicates: usize,
    aliases: usize,
}

impl Directory {
    fn new(original: &str) -> Self {
        let path = PathBuf::from(original);
        Directory {
            original: path.clone(),
            canonical: path.canonicalize().ok(),
        }
    }
}

fn gather_counts(directories: &[Directory]) -> Vec<DirectoryWithCounts> {
    use std::collections::HashMap;

    // Count how many times each original path appears
    let mut duplicate_counts = HashMap::new();
    for dir in directories {
        *duplicate_counts.entry(&dir.original).or_insert(0) += 1;
    }

    // Count how many directories share each canonical path
    let mut canonical_counts = HashMap::new();
    for dir in directories {
        if let Some(canonical) = &dir.canonical {
            *canonical_counts.entry(canonical).or_insert(0) += 1;
        }
    }

    directories
        .iter()
        .map(|dir| {
            let duplicates = *duplicate_counts.get(&dir.original).unwrap_or(&0);
            // this is so heavy :-((
            let aliases = dir
                .canonical
                .as_ref()
                .and_then(|c| canonical_counts.get(c))
                .copied()
                .unwrap_or(0usize)
                .saturating_sub(1);

            DirectoryWithCounts {
                original: dir.original.clone(),
                canonical: dir.canonical.clone(),
                duplicates,
                aliases,
            }
        })
        .collect()
}

fn print_dir_info(dir: &DirectoryWithCounts, lineno: usize) {
    let num = format!("{:>2}", lineno).dimmed();
    let path_str = dir.original.display().to_string();

    if dir.original.exists() {
        print!("{} {} ", num, path_str.green());

        if dir.duplicates > 1 || dir.aliases > 1 {
            print!(
                "{}",
                format!("(duplicates ×{}, aliases ×{})", dir.duplicates, dir.aliases)
                    .yellow()
                    .bold()
            );
        } else {
            print!("{}", "(unique)".green().dimmed());
        }

        if let Some(canonical) = &dir.canonical {
            if canonical != &dir.original {
                print!(" {}", format!("→ {}", canonical.display()).dimmed());
            }
        }

        println!();
    } else {
        println!("{} {} {}", num, path_str.red(), "(missing)".red().dimmed());
    }
}

fn print_directories(directories: &[Directory]) {
    let dirs_with_counts = gather_counts(directories);
    for (i, dir) in dirs_with_counts.iter().enumerate() {
        print_dir_info(dir, i + 1);
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
