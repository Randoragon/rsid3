use std::ffi::OsStr;
use std::path::Path;
use std::fs::{create_dir_all, copy};
use tempfile::NamedTempFile;
use std::process::{Command, Output};

/// Path to the directory containing preset samples.
const SAMPLE_EMPTY: &str = "tests/samples/sample.mp3";
/// Path to the directory for storing temporary files constructed and operated on in integration tests.
const SAMPLES_TMPDIR: &str = "tests/samples/tmp/";
/// Path to the rsid3 executable.
const PROGRAM_PATH: &str = "target/debug/rsid3";

pub struct TestFile {
    file: NamedTempFile,
}

impl TestFile {
    /// Creates a new tagless test file.
    pub fn empty() -> Self {
        create_dir_all(SAMPLES_TMPDIR).unwrap();
        let file = NamedTempFile::new_in(SAMPLES_TMPDIR).unwrap();
        copy(SAMPLE_EMPTY, file.path()).unwrap();
        TestFile {
            file,
        }
    }

    /// Returns the path to the test file.
    pub fn path(&self) -> &Path {
        self.file.path()
    }
}

pub fn rsid3_run(args: &[impl AsRef<OsStr>]) -> Output {
    let mut cmd = Command::new(PROGRAM_PATH);
    cmd.args(args);
    println!("Command: {:?}", cmd);
    let output = cmd.output().unwrap();
    println!("Status:  {:?}", output.status);
    println!("Stdout:  {:?}", String::from_utf8(output.stdout.clone()).unwrap());
    println!("Stderr:  {:?}", String::from_utf8(output.stderr.clone()).unwrap());
    output
}
