use directories::BaseDirs;
use std::path::PathBuf;

pub fn sourcetree_settings_path() -> Option<PathBuf> {
    if let Some(base_dirs) = BaseDirs::new() {
        let data_local_dir_path = base_dirs.data_local_dir();
        let source_tree_path = data_local_dir_path.join("Atlassian").join("SourceTree");
        return Some(source_tree_path);
    }
    None
}

pub fn sourcetree_exec_path() -> Option<PathBuf> {
    if let Some(base_dirs) = BaseDirs::new() {
        let data_local_dir_path = base_dirs.data_local_dir();
        let source_tree_path = data_local_dir_path
            .join("SourceTree")
            .join("SourceTree.exe");
        return Some(source_tree_path);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_sourcetree_settings_path() {
        let settings_path = sourcetree_settings_path();
        assert_ne!(settings_path, None);

        let settings_path = sourcetree_settings_path().unwrap();
        assert!(settings_path.is_dir());
    }
}
