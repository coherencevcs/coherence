use std::{
    fs,
    io::{self, ErrorKind},
    path::PathBuf,
};

use crate::{log_err, log_ok, utils::pathbuf_to_string};

pub fn main(path: &Option<PathBuf>) -> anyhow::Result<()> {
    let pathbuf = path.to_owned().unwrap_or(PathBuf::from("."));
    let dot_path = pathbuf_to_string(&pathbuf)?;

    match create_coherence_fs(&pathbuf) {
        Ok(_) => (),
        Err(e) => {
            if e.kind() == ErrorKind::AlreadyExists {
                return log_ok!(
                    "warn",
                    "Unable to initialize Coherence repository in: {}: repository already exists",
                    &dot_path
                );
            } else if e.kind() == ErrorKind::NotFound {
                return log_ok!(
                    "warn",
                    "Unable to intialize Coherence repository in: {}: directory not found",
                    &dot_path
                );
            };

            return log_err!(
                "error",
                "Unable to initialize Coherence repository in: {}: {}",
                &dot_path,
                e.to_string()
            );
        }
    };

    log_ok!(
        "info",
        "Initialized empty Coherence repository in: {}",
        &dot_path
    )
}

pub fn create_coherence_fs(path: &PathBuf) -> anyhow::Result<(), io::Error> {
    fs::create_dir_all(path.join(".chr").join("branches").join("main"))?;
    fs::write(path.join(".chr").join("config.toml"), "")?; // TODO: Fill out default values
    fs::write(
        path.join(".chr")
            .join("branches")
            .join("main")
            .join("branch.toml"),
        "",
    )?; // TODO: Fill out default values
    fs::write(path.join(".chr").join("proto.db"), "")?; // TODO: Actually create SQLite database

    Ok(())
}
