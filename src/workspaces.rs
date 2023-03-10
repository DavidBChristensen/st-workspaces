use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::paths::sourcetree_settings_path;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Workspaces {
    pub workspaces: Vec<Workspace>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    pub name: String,
    pub repo_paths: Vec<String>,
}

impl Workspace {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            repo_paths: Default::default(),
        }
    }
}

pub fn workspace_path() -> Option<PathBuf> {
    let settings_path = sourcetree_settings_path();
    if settings_path.is_none() {
        return settings_path;
    }

    Some(settings_path.unwrap().join("st-workspaces.ron"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_list_of_workspaces() {
        let workspace_path = workspace_path();
        assert_ne!(workspace_path, None);

        let workspace_path = workspace_path.unwrap();
        assert_eq!(workspace_path.extension().unwrap().to_str().unwrap(), "ron");
    }

    #[test]
    fn should_serialize_workspace() {
        let mut spaces = Workspaces::default();
        let mut space = Workspace::new("First Workspace");
        space.repo_paths.push("C:\\fake\\path0".to_owned());
        space.repo_paths.push("C:\\fake\\path1".to_owned());
        space.repo_paths.push("C:\\fake\\path2".to_owned());
        spaces.workspaces.push(space);

        let mut space = Workspace::new("Second Workspace");
        space.repo_paths.push("C:\\fake\\path3".to_owned());
        space.repo_paths.push("C:\\fake\\path4".to_owned());
        space.repo_paths.push("C:\\fake\\path5".to_owned());
        spaces.workspaces.push(space);

        let serialized_spaces = ron::to_string(&spaces).unwrap();
        let spaces: Workspaces = ron::from_str(&serialized_spaces).unwrap();
        let first_workspace = &spaces.workspaces[0];
        assert_eq!(first_workspace.name, "First Workspace".to_owned());
        assert_eq!(first_workspace.repo_paths[0], "C:\\fake\\path0".to_owned());
        assert_eq!(first_workspace.repo_paths[1], "C:\\fake\\path1".to_owned());
        assert_eq!(first_workspace.repo_paths[2], "C:\\fake\\path2".to_owned());

        let second_workspace = &spaces.workspaces[1];
        assert_eq!(second_workspace.name, "Second Workspace".to_owned());
        assert_eq!(second_workspace.repo_paths[0], "C:\\fake\\path3".to_owned());
    }
}
