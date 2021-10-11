extern crate walkdir;
use walkdir::WalkDir;

extern crate colored;
use colored::*;

pub struct Config<'a> {
    pattern: &'a str,
    count: bool,
    ignore_case: bool,
    no_color: bool,
    //regexp
}

impl<'a> Config<'a> {
    pub fn new(pattern: &'a str, count: bool, ignore_case: bool, no_color: bool) -> Config<'a> {
        Config {
            pattern,
            count,
            ignore_case,
            no_color,
        }
    }
}

pub fn rlocate(cfg: Config) {
    let mut matches: u32 = 0;
    for entry in WalkDir::new("/").into_iter().filter_map(|e| e.ok()) {
        let mut pattern = cfg.pattern.to_string();
        let mut path = entry.path().to_str().unwrap_or("").to_string();
        if cfg.ignore_case {
            pattern = pattern.to_lowercase();
            path = path.to_lowercase();
        }

        if path.contains(&pattern) {
            if cfg.count {
                matches += 1;
            } else {
                if cfg.no_color {
                    println!("{}", entry.path().display());
                } else {
                    let pos = path.find(&pattern).unwrap();
                    println!(
                        "{}{}{}",
                        &entry.path().to_str().unwrap_or("").to_string()[..pos],
                        &entry.path().to_str().unwrap_or("").to_string()[pos..pos + pattern.len()]
                            .green(),
                        &entry.path().to_str().unwrap_or("").to_string()[pos + pattern.len()..]
                    );
                }
            }
        }
    }

    if cfg.count {
        println!("{}", matches);
    }
}
