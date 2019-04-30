use std::process::Command;
use assert_cmd::prelude::*;
use tempfile::tempdir;

#[test]
fn hidden_reqtangle_directory_is_created_if_not_exist() -> Result<(), Box<std::error::Error>> {
    let working_dir = tempdir()?;
    let mut cmd = Command::cargo_bin("reqtangle")?;
    cmd
        .current_dir(&working_dir)
        .arg("gather");
    cmd.assert().success();
    let hidden_reqtangle_dir = working_dir.path().canonicalize()?.join(".reqtangle");
    assert!(hidden_reqtangle_dir.exists());
    working_dir.close()?;
    Ok(())
}