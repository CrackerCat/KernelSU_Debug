#[allow(clippy::wildcard_imports)]
use crate::utils::*;
use crate::{
    assets, defs,
    sepolicy,
};

use anyhow::{anyhow, Context, Result};
use is_executable::is_executable;
use log::{info, warn};
use std::{
    env::var as env_var,
    path::Path,
    process::Command
};

#[cfg(unix)]
use std::os::unix::process::CommandExt;

pub fn load_sepolicy_rule() -> Result<()> {
    let rule_file = Path::new(defs::COMMON_DIR).join("sepolicy.rule");
    if !rule_file.exists() {
        return Ok(());
    }
    info!("load policy: {}", &rule_file.display());

    if sepolicy::apply_file(&rule_file).is_err() {
        warn!("Failed to load sepolicy.rule for {}", &rule_file.display());
    }
    Ok(())
}

fn exec_script<T: AsRef<Path>>(path: T, wait: bool) -> Result<()> {
    info!("exec {}", path.as_ref().display());

    let mut command = &mut Command::new(assets::BUSYBOX_PATH);
    #[cfg(unix)]
    {
        command = command.process_group(0);
        command = unsafe {
            command.pre_exec(|| {
                // ignore the error?
                switch_cgroups();
                Ok(())
            })
        };
    }
    command = command
        .current_dir(path.as_ref().parent().unwrap())
        .arg("sh")
        .arg(path.as_ref())
        .env("ASH_STANDALONE", "1")
        .env("KSU", "true")
        .env("KSU_KERNEL_VER_CODE", crate::ksu::get_version().to_string())
        .env("KSU_VER_CODE", defs::VERSION_CODE)
        .env("KSU_VER", defs::VERSION_NAME)
        .env(
            "PATH",
            format!(
                "{}:{}",
                env_var("PATH").unwrap(),
                defs::BINARY_DIR.trim_end_matches('/')
            ),
        );

    let result = if wait {
        command.status().map(|_| ())
    } else {
        command.spawn().map(|_| ())
    };
    result.map_err(|err| anyhow!("Failed to exec {}: {}", path.as_ref().display(), err))
}

pub fn exec_post_fs_data() -> Result<()> {
    let post_fs_data = Path::new(defs::COMMON_DIR).join("post-fs-data.sh");
    if !post_fs_data.exists() {
        return Ok(());
    }

    return exec_script(&post_fs_data, true)
}

pub fn exec_common_scripts(dir: &str, wait: bool) -> Result<()> {
    let script_dir = Path::new(defs::ADB_DIR).join(dir);
    if !script_dir.exists() {
        info!("{} not exists, skip", script_dir.display());
        return Ok(());
    }

    let dir = std::fs::read_dir(&script_dir)?;
    for entry in dir.flatten() {
        let path = entry.path();

        if !is_executable(&path) {
            warn!("{} is not executable, skip", path.display());
            continue;
        }

        exec_script(path, wait)?;
    }

    Ok(())
}

/// execute every modules' [stage].sh (service.sh, boot-completed.sh)
pub fn exec_stage_scripts(stage: &str) -> Result<()> {
    let service = Path::new(defs::COMMON_DIR).join(format!("{stage}.sh"));
    if !service.exists() {
        return Ok(());
    }

    return exec_script(&service, false)
}

pub fn load_system_prop() -> Result<()> {
    let system_prop = Path::new(defs::COMMON_DIR).join("system.prop");
    if !system_prop.exists() {
        return Ok(());
    }
    info!("load system.prop");

    // resetprop -n --file system.prop
    Command::new(assets::RESETPROP_PATH)
        .arg("-n")
        .arg("--file")
        .arg(&system_prop)
        .status()
        .with_context(|| format!("Failed to exec {}", system_prop.display()))?;

    Ok(())
}