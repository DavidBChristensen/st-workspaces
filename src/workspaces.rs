use std::path::PathBuf;

use crate::paths::sourcetree_settings_path;

pub struct Workspaces {
    pub workspaces: Vec<Workspace>,
}

pub struct Workspace {
    pub name: String,
    pub repos: Vec<String>,
}

impl Workspace {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            repos: Default::default(),
        }
    }
}

pub fn workspace_path() -> Option<PathBuf> {
    let settings_path = sourcetree_settings_path();
    if settings_path.is_none() {
        return settings_path;
    }

    Some(settings_path.unwrap().join("st-workspaces.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_list_of_workspaces() {
        let workspace_path = workspace_path();
        assert_ne!(workspace_path, None);
    }
}
