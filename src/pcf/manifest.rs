use crate::pcf::{FileHandler, Version, find_file, update_file};
use std::path;

const MANIFEST_FILE_NAME: &str = "ControlManifest.Input.xml";

pub struct ManifestFile {
    path: path::PathBuf,
    pub version: Version,
    line: usize,
    col: usize,
    len: usize,
}

impl FileHandler for ManifestFile {
    fn get(path: path::PathBuf) -> Option<Self> where Self: Sized {
        match find_file(path, MANIFEST_FILE_NAME) {
            Some(file_path) => {
                if let Some(read_result) = read_xml_file(&file_path) {
                    if let Some(version) = Version::parse(&read_result.0) {
                        return Some(ManifestFile {
                            path: file_path,
                            version: version,
                            line: read_result.1,
                            col: read_result.2,
                            len: read_result.0.len()
                        })
                    }
                }
                None
            },
            _ => None
        }
    }

    fn show_status(&self) {
        println!("{}: {}", MANIFEST_FILE_NAME, self.version);
    }

    fn read_version(path: &path::PathBuf) -> Option<String> {
        match read_xml_file(path) {
            Some(result) => Some(result.0),
            None => None
        }
    }

    fn update_version(&self, version: &Version) -> Result<(), ()> {
       update_file(&self.path, &version.to_string(), self.line, self.col, self.len)
    }
}

fn read_xml_file(path: &path::PathBuf) -> Option<(String, usize, usize)> {
    match std::fs::read_to_string(&path) {
        Ok(content) => {
            let mut line_num: usize = 0;
            let mut control_open = false;
            for line in content.lines() {
                if line.to_lowercase().contains("<control") {
                    control_open = true;
                }

                if line.to_lowercase().contains(">") && control_open {
                    break;
                }

                if control_open {
                    if let Some(idx_start) = line.to_lowercase().find("version=") {
                        let substr = &line[idx_start + 8..];
                        let mut char_vec: Vec<char> = Vec::new();
                        for c in substr.chars() {
                            if c == '\"' && char_vec.len() != 0 {
                                break;
                            }
                            
                            if c.is_numeric() || c == '.' {
                                char_vec.push(c);
                            }
                        }
    
                        return Some((char_vec.iter().collect(), line_num, idx_start + 9));
                    }
                }

                line_num += 1;
            }

            None
        },
        Err(e) => {
            println!("Error reading file {:?}", e);
            None
        }
    }
}

