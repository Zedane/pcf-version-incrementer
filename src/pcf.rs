use std::fmt;
use std::path;

pub mod package;
pub mod solution;
pub mod manifest;

pub trait FileHandler {
    fn get(path: path::PathBuf) -> Option<Self> where Self: Sized;
    fn show_status(&self);
    fn read_version(path: &path::PathBuf) -> Option<String>;
    fn update_version(&self, version: &Version) -> Result<(), ()>;
}


#[derive(Debug, Clone)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl Version {
    pub fn parse(str: &str) -> Option<Self> {
        let sections: Vec<&str> = str.split('.').collect();
        if sections.len() >= 3 {
            return Some(Version {
                major: sections[0].parse::<u16>().unwrap(),
                minor: sections[1].parse::<u16>().unwrap(),
                patch: sections[2].parse::<u16>().unwrap(),
            });
        } else if sections.len() == 2 {
            return Some(Version {
                major: sections[0].parse::<u16>().unwrap(),
                minor: sections[1].parse::<u16>().unwrap(),
                patch: 0,
            });
        } else if sections.len() == 1 {
            return Some(Version {
                major: sections[0].parse::<u16>().unwrap(),
                minor: 0,
                patch: 0,
            });
        }

        None
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }

    pub fn set_major(&mut self, val: u16) {
        self.major = val;
        self.minor = 0;
        self.patch = 0;
    }

    pub fn set_minor(&mut self, val: u16) {
        self.minor = val;
        self.patch = 0;
    }

    pub fn set_patch(&mut self, val: u16) {
        self.patch = val;
    }

    
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}


pub fn find_file(dir: path::PathBuf, filename: &str) -> Option<path::PathBuf> {
    if let Ok(elements) = path::Path::new(&dir).read_dir() {
        for element in elements {
            if let Ok(entry) = element {
                let path = entry.path();

                // println!("> {}", &path.display());

                if path.is_file() {
                    if let Some(file_name) = extract_file_name(&path) {
                        if file_name == filename {
                            return Some(path);
                        }
                    }
                }

                if path.is_dir() {
                    if let Some(directory_name) = extract_file_name(&path) {
                        if directory_name != "node_modules" {
                            if let Some(path) = find_file(path.clone(), filename) {
                                if let Some(file_name) = extract_file_name(&path) {
                                    if file_name == filename {
                                        return Some(path);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

fn extract_file_name(path: &path::PathBuf) -> Option<String> {
    if let Some(file_name) = path.file_name() {
        if let Some(file) = file_name.to_str() {
            return Some(String::from(file));
        }
    }
    None
}

pub fn update_file(path: &path::PathBuf, version: &str, line: usize, col: usize, len: usize) -> Result<(), ()> {
    match std::fs::read_to_string(path) {
        Ok(contents) => {
            let mut lines: Vec<String> = contents.lines().map(String::from).collect();
            lines[line].replace_range(col..col + len, version);

            return match std::fs::write(path, lines.join("\r\n")) {
                Ok(_) => Ok(()),
                Err(_) => Err(())
            };
        },
        Err(error) => {
            eprintln!("{:?}", error);
            Err(())
        }
    }
}
