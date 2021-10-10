extern crate walkdir;
use walkdir::WalkDir;

pub struct Config<'a> {
    pattern: &'a str,
    count: bool,
    ignore_case: bool,
    //regexp
}

impl<'a> Config<'a> {
    pub fn new(pattern: &'a str, count: bool, ignore_case: bool) -> Config<'a> {
        Config {
            pattern,
            count,
            ignore_case,
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
                println!("{}", path);
            }
        }
    }
    if cfg.count {
        println!("{}", matches);
    }
}
