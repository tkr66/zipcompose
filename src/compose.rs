use std::{
    fs::{self},
    io::Write,
    path::Path,
};

use zip::write::SimpleFileOptions;

use super::manifest::{FileMapping, Manifest};

pub fn check(manifest: &Manifest, name: &str) -> Result<(), std::io::Error> {
    let archive = manifest.archives.get(name).expect("key not found");
    if let Some(parent) = Path::parent(Path::new(archive.filename.as_str())) {
        if parent.to_str().unwrap() != "" && !parent.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "The output directory does not exist",
            ));
        }
    }
    for entry in &archive.entries {
        for file in &entry.files {
            match file {
                FileMapping::Source(src) => {
                    if !Path::new(src).exists() {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::NotFound,
                            "The source does not exist",
                        ));
                    }
                }
                FileMapping::SourceWithDestination { src, .. } => {
                    if !Path::new(src).exists() {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::NotFound,
                            "The source does not exist",
                        ));
                    }
                }
                FileMapping::Glob(_) => todo!(),
            }
        }
    }

    Ok(())
}

pub fn check_all(manifest: &Manifest) -> Result<(), std::io::Error> {
    for k in manifest.archives.keys() {
        check(manifest, k.as_str())?;
    }

    Ok(())
}

pub fn run(manifest: &Manifest, name: &str) -> Result<(), std::io::Error> {
    let archive = manifest
        .archives
        .get(name)
        .expect("The specified archive does not found in the compose yaml");
    let path = std::path::Path::new(&archive.filename);
    let file = std::fs::File::create(path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    for e in &archive.entries {
        let file_option = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o644);
        let mut root_in_entries = Path::new(&e.dest_dir);
        if root_in_entries.starts_with("./") {
            root_in_entries = root_in_entries.strip_prefix("./").unwrap();
        }
        for f in &e.files {
            match f {
                FileMapping::Source(s) => {
                    let filename = Path::new(s)
                        .file_name()
                        .expect("The filename must not be null");
                    let file_in_zip = root_in_entries.join(filename);
                    let valid_path = file_in_zip.to_str().expect("Invalid utf8 string");
                    zip.start_file(valid_path, file_option)?;
                    let content = fs::read(s.as_str())?;
                    zip.write_all(&content)?;
                }
                FileMapping::SourceWithDestination { src, dest } => {
                    let filename = Path::new(dest)
                        .file_name()
                        .expect("The filename must not be null");
                    let file_in_zip = root_in_entries.join(filename);
                    let valid_path = file_in_zip.to_str().expect("Invalid utf8 string");
                    zip.start_file(valid_path, file_option)?;
                    let content = fs::read(src.as_str())?;
                    zip.write_all(&content)?;
                }
                FileMapping::Glob(_) => todo!(),
            }
        }
    }

    zip.finish()?;

    Ok(())
}

pub fn run_all(manifest: &Manifest) -> Result<(), std::io::Error> {
    for k in manifest.archives.keys() {
        run(manifest, k)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::*;
    use std::collections::HashMap;

    #[test]
    fn check_not_found_output_dir() {
        let name = String::from("test");
        let mut map: HashMap<String, Archive> = HashMap::new();
        let archive = Archive {
            filename: "/dir/not/found/test.zip".to_string(),
            entries: Vec::new(),
        };
        map.insert(name.clone(), archive);
        let manifest = Manifest { archives: map };
        assert!(check(&manifest, name.as_str())
            .is_err_and(|e| e.kind() == std::io::ErrorKind::NotFound));
    }

    #[test]
    fn check_not_found_source() {
        let name = "test".to_string();
        let mut map: HashMap<String, Archive> = HashMap::new();
        let archive = Archive {
            filename: "test.zip".to_string(),
            entries: vec![FilesWithDestination {
                dest_dir: "".to_string(),
                files: vec![FileMapping::Source("src/not/found".to_string())],
            }],
        };
        map.insert(name.clone(), archive);
        let manifest = Manifest { archives: map };
        assert!(check(&manifest, name.as_str())
            .is_err_and(|e| e.kind() == std::io::ErrorKind::NotFound));
    }
}
