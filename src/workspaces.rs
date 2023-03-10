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

    Some(settings_path.unwrap().join("st-workspaces.json"))
}

fn write_workspace(path: &PathBuf, workspaces: &Workspaces) -> anyhow::Result<()> {
    let contents = serde_json::to_string_pretty(&workspaces)?;
    std::fs::write(path, contents)?;
    Ok(())
}

fn read_workspace(path: &PathBuf) -> anyhow::Result<Workspaces> {
    let contents = std::fs::read_to_string(path)?;
    let workspaces: Workspaces = serde_json::from_str(&contents).unwrap();
    Ok(workspaces)
}

pub fn write_workspace_to_disk(workspaces: &Workspaces) -> anyhow::Result<()> {
    let path = workspace_path().unwrap();
    write_workspace(&path, workspaces)?;
    Ok(())
}

pub fn read_workspace_from_disk() -> anyhow::Result<Workspaces> {
    let path = workspace_path().unwrap();
    let workspaces = read_workspace(&path)?;
    Ok(workspaces)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_path() -> PathBuf {
        let path = sourcetree_settings_path().unwrap();
        path.join("st-workspaces-test.json")
    }

    fn create_test_workspaces() -> Workspaces {
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
        spaces
    }

    #[test]
    fn should_get_list_of_workspaces() {
        let workspace_path = workspace_path();
        assert_ne!(workspace_path, None);

        let workspace_path = workspace_path.unwrap();
        assert_eq!(
            workspace_path.extension().unwrap().to_str().unwrap(),
            "json"
        );
    }

    #[test]
    fn should_serialize_workspace() {
        let spaces = create_test_workspaces();
        let serialized_spaces = serde_json::to_string(&spaces).unwrap();
        let spaces: Workspaces = serde_json::from_str(&serialized_spaces).unwrap();
        let first_workspace = &spaces.workspaces[0];
        assert_eq!(first_workspace.name, "First Workspace".to_owned());
        assert_eq!(first_workspace.repo_paths[0], "C:\\fake\\path0".to_owned());
        assert_eq!(first_workspace.repo_paths[1], "C:\\fake\\path1".to_owned());
        assert_eq!(first_workspace.repo_paths[2], "C:\\fake\\path2".to_owned());

        let second_workspace = &spaces.workspaces[1];
        assert_eq!(second_workspace.name, "Second Workspace".to_owned());
        assert_eq!(second_workspace.repo_paths[0], "C:\\fake\\path3".to_owned());
    }

    #[test]
    fn should_persist_workspace() -> anyhow::Result<()> {
        let spaces = create_test_workspaces();
        let test_path = test_path();
        write_workspace(&test_path, &spaces)?;
        let spaces = read_workspace(&test_path)?;
        let first_workspace = &spaces.workspaces[0];
        assert_eq!(first_workspace.name, "First Workspace".to_owned());
        assert_eq!(first_workspace.repo_paths[0], "C:\\fake\\path0".to_owned());
        assert_eq!(first_workspace.repo_paths[1], "C:\\fake\\path1".to_owned());
        assert_eq!(first_workspace.repo_paths[2], "C:\\fake\\path2".to_owned());

        let second_workspace = &spaces.workspaces[1];
        assert_eq!(second_workspace.name, "Second Workspace".to_owned());
        assert_eq!(second_workspace.repo_paths[0], "C:\\fake\\path3".to_owned());
        Ok(())
    }
}
