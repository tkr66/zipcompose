use serde::Deserialize;
use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

#[derive(Deserialize, Debug)]
pub struct Manifest {
    pub archives: HashMap<String, Archive>,
}

#[derive(Deserialize, Debug)]
pub struct Archive {
    pub filename: String,
    pub entries: Vec<FilesWithDestination>,
}

#[derive(Deserialize, Debug)]
pub struct FilesWithDestination {
    pub dest_dir: String,
    pub files: Vec<FileMapping>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum FileMapping {
    Source(String),
    SourceWithDestination { src: String, dest: String },
}

pub fn read(path: &Path) -> Result<Manifest, std::io::Error> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let m = serde_yml::from_reader(reader).unwrap();

    Ok(m)
}

#[test]
fn deserialize_files_with_destination() {
    let yaml = "
        dest_dir: .
        files:
            - main.rs
            - src: main.rs
              dest: renamed_main.rs
            - lib.rs
            - src: lib.rs
              dest: renamed_lib.rs
        ";
    let t: FilesWithDestination = serde_yml::from_str(yaml).unwrap();
    assert_eq!(t.dest_dir, ".");
    assert_eq!(t.files.len(), 4);
    assert_eq!(t.files[0], FileMapping::Source("main.rs".to_string()));
    assert_eq!(
        t.files[1],
        FileMapping::SourceWithDestination {
            src: "main.rs".to_string(),
            dest: "renamed_main.rs".to_string()
        }
    );
    assert_eq!(t.files[2], FileMapping::Source("lib.rs".to_string()));
    assert_eq!(
        t.files[3],
        FileMapping::SourceWithDestination {
            src: "lib.rs".to_string(),
            dest: "renamed_lib.rs".to_string()
        }
    );
}

#[test]
fn deserialize_archive() {
    let yaml = "
        filename: test.zip
        entries:
        ";
    let t: Archive = serde_yml::from_str(yaml).unwrap();
    assert_eq!(t.filename, "test.zip");
    assert!(t.entries.is_empty());
}

#[test]
fn deserialize_manifest() {
    let yaml = "
        archives:
            foo:
                filename: foo.zip
                entries:
            bar:
                filename: bar.zip
                entries:
        ";
    let t: Manifest = serde_yml::from_str(yaml).unwrap();
    let keys: Vec<&String> = t.archives.keys().collect();
    assert_eq!(keys.len(), 2);
    assert!(t.archives.contains_key("foo"));
    assert!(t.archives.contains_key("bar"));
}
