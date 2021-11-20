enum PackageManager {
    Npm,
    Yarn,
    Pnpm,
}

// A function to guess package manager by looking for lock file in current directory only
// If yarn.lock is found, it's likely to be a yarn project
// If package-lock.json is found, it's likely to be a npm project
// If pnpm-lock.yaml is found, it's likely to be a pnpm project
// If none of the above is found, return None
pub fn guess_package_manager(dir: &PathBuf) -> Option<PackageManager> {
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

    None
}
