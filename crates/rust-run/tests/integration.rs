use ::{
    expect_test::{expect, Expect},
    std::{fs, path::PathBuf, process::Command},
};

#[test]
fn examples() {
    fn examples_dir(dir: PathBuf) {
        for entry in ::std::fs::read_dir("examples").unwrap() {
            let entry = entry.unwrap();
            let entry_type = entry.file_type().unwrap();
            if entry_type.is_file() {
                examples_file(entry.path());
            } else if entry_type.is_dir() {
                examples_dir(entry.path());
            }
        }
    }

    fn examples_file(path: PathBuf) {
        let mut command = Command::new("cargo");
        command.args(["run", "--"]).arg(path);

        let child = command.spawn().unwrap();

        dbg!(child.wait_with_output().unwrap());
    }

    examples_dir("examples".into());
}
