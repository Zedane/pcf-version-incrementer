use crate::pcf::{FileHandler, Version, find_file, update_file};
use std::path;

const SOLUTION_FILE_NAME: &str = "Solution.xml";

pub struct SolutionFile {
    path: path::PathBuf,
    version: Version,
    line: usize,
    col: usize,
    len: usize,
}

impl FileHandler for SolutionFile {
    fn get(path: path::PathBuf) -> Option<Self> where Self: Sized {
        match find_file(path, SOLUTION_FILE_NAME) {
            Some(file_path) => {
                if let Some(read_result) = read_xml_file(&file_path) {
                    if let Some(version) = Version::parse(&read_result.0) {
                        // println!("{:?}", version);
                        return Some(SolutionFile {
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
        println!("{}: {}", SOLUTION_FILE_NAME, self.version);
    }

    fn read_version(path: &path::PathBuf) -> Option<String> {
        match read_xml_file(path) {
            Some(result) => Some(result.0),
            None => None
        }
    }

    fn update_version(&self, version: &Version) -> Result<(), ()> {
        let version_str = format!("{}.{}", version.major, version.minor);
        update_file(&self.path, &version_str, self.line, self.col, self.len)
    }
}

fn read_xml_file(path: &path::PathBuf) -> Option<(String, usize, usize)> {
    match std::fs::read_to_string(&path) {
        Ok(content) => {
            let mut line_num: usize = 0;
            for line in content.lines() {
                if let Some(idx_start) = line.to_lowercase().find("<version>") {
                    if let Some(idx_end) = line.to_lowercase().find("</version>") {
                        return Some((String::from(&line[idx_start + 9..idx_end]), line_num, idx_start + 9));
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