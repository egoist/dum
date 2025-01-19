use std::path::Path;
use std::str::FromStr;

use anyhow::{anyhow, Error};

use crate::prompt;

pub enum PackageManager {
    Yarn,
    Npm,
    Pnpm,
    Bun,
}

impl ToString for PackageManager {
    fn to_string(&self) -> String {
        match self {
            PackageManager::Yarn => "yarn".to_string(),
            PackageManager::Npm => "npm".to_string(),
            PackageManager::Pnpm => "pnpm".to_string(),
            PackageManager::Bun => "bun".to_string(),
        }
    }
}

impl FromStr for PackageManager {
    type Err = Error;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
            "yarn" => Ok(PackageManager::Yarn),
            "npm" => Ok(PackageManager::Npm),
            "pnpm" => Ok(PackageManager::Pnpm),
            "bun" => Ok(PackageManager::Bun),
            _ => Err(anyhow!("Parse package manager error")),
        }
    }
}

// A function to guess package manager by looking for lock file in current directory only
// If yarn.lock is found, it's likely to be a yarn project
// If package-lock.json is found, it's likely to be a npm project
// If pnpm-lock.yaml is found, it's likely to be a pnpm project
// If none of the above is found, return None
pub fn guess_package_manager(dir: &Path) -> Option<PackageManager> {
    let lock_file = dir.join("yarn.lock");
    if lock_file.exists() {
        return Some(PackageManager::Yarn);
    }

    let lock_file = dir.join("package-lock.json");
    if lock_file.exists() {
        return Some(PackageManager::Npm);
    }

    let lock_file = dir.join("pnpm-lock.yaml");
    if lock_file.exists() {
        return Some(PackageManager::Pnpm);
    }

    // bun supports both bun.lockb and bun.lock
    let lock_file = dir.join("bun.lockb");
    if lock_file.exists() {
        return Some(PackageManager::Bun);
    }

    let lock_file = dir.join("bun.lock");
    if lock_file.exists() {
        return Some(PackageManager::Bun);
    }

    let items = vec!["pnpm", "npm", "yarn", "bun"];
    match prompt::select("Which package manager do you want to use?", items) {
        Some(pm) => PackageManager::from_str(&pm).ok(),
        None => None,
    }
}
