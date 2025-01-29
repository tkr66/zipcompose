use std::{
    fs::{File, OpenOptions},
    io::{BufReader, Write},
    path::{Path, PathBuf},
    process::Command,
};

use zip::ZipArchive;

#[test]
fn test_run() {
    let contents = b"
archives:
  test:
    filename: test.zip
    entries:
      - dest_dir: .
        files:
        - README.md

      - dest_dir: ./rs
        files:
        - ./src/main.rs
        - src: ./src/command.rs
          dest: renamed.rs

      - dest_dir: ./rs/cargo
        files:
        - Cargo.toml
        - Cargo.lock
        ";
    with_file("test.yaml", contents, |p| {
        let bin = PathBuf::from(env!("CARGO_BIN_EXE_zipcompose"));
        let out = Command::new(bin)
            .args(["-f", p.to_str().unwrap(), "run", "-a", "test"])
            .output()
            .expect("Failed to execute command");

        println!("{}", String::from_utf8_lossy(&out.stdout));
        println!("{}", String::from_utf8_lossy(&out.stderr));

        let zip = read_zip("test.zip").expect("Failed to read zip");
        assert_eq!(5, zip.len());
        assert!(zip.index_for_path("README.md").is_some());
        assert!(zip.index_for_path("rs/main.rs").is_some());
        assert!(zip.index_for_path("rs/renamed.rs").is_some());
        assert!(zip.index_for_path("rs/cargo/Cargo.toml").is_some());
        assert!(zip.index_for_path("rs/cargo/Cargo.lock").is_some());
        if Path::new("test.zip").try_exists().is_ok() {
            let _ = std::fs::remove_file("test.zip");
        }
    });
}

fn with_file<F>(path: &str, contents: &[u8], func: F)
where
    F: Fn(&Path),
{
    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .unwrap();
    match f.write(contents) {
        Ok(b) => println!("Sucessfully written bytes: {}", b),
        Err(e) => eprintln!("An error occured while writing: {}", e),
    };

    let p = Path::new(path);
    func(p);
    if p.try_exists().is_ok() {
        let _ = std::fs::remove_file(p);
    }
}

fn read_zip(file: &str) -> std::io::Result<ZipArchive<BufReader<File>>> {
    let f = File::open(file)?;
    let rd = BufReader::new(f);
    let zip = ZipArchive::new(rd)?;
    Ok(zip)
}
