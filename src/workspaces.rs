use std::{path::PathBuf, collections::HashMap};

use anyhow::bail;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::paths::sourcetree_settings_path;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Workspaces {
    pub current_workspace: Uuid,
    pub workspaces: HashMap<Uuid, Workspace>,
}

impl Workspaces{
    pub fn current_workspace(& self) -> Option<& Workspace> {
        self.workspaces.get(&self.current_workspace)
    }

    pub fn current_workspace_mut(&mut self) -> Option<&mut Workspace> {
        self.workspaces.get_mut(&self.current_workspace)
    }

    pub fn force_valid_workspace(&mut self) {
        if !self.workspaces.is_empty() && self.current_workspace.is_nil(){
            self.current_workspace = *self.workspaces.iter().next().unwrap().0;
        }
    }

    pub fn write(&self) -> anyhow::Result<()> {
        let Some(path) = Workspaces::path() else { 
            bail!("Error getting workspace path for reading."); 
        };

        write_to_path(&path, self)?;
        Ok(())
    }

    pub fn path() -> Option<PathBuf> {
        let Some(settings_path) = sourcetree_settings_path() else { 
            return None; 
        };

        Some(settings_path.join("st-workspaces.json"))
    }

    pub fn read() -> anyhow::Result<Workspaces> {
        let Some(path) = Workspaces::path() else { 
            bail!("Error getting workspace path for reading."); 
        };

        let workspaces = read_from_path(&path)?;
        Ok(workspaces)
    }

}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Workspace {
    pub uuid: Uuid,
    pub name: String,
    pub repo_paths: Vec<String>,
}

impl Ord for Workspace{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Workspace{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Workspace {
    pub fn new(name: &str, uuid : Uuid) -> Self {
        Self {
            uuid, 
            name: name.to_string(),
            repo_paths: Default::default(),
        }
    }
}

fn write_to_path(path: &PathBuf, workspaces: &Workspaces) -> anyhow::Result<()> {
    let contents = serde_json::to_string_pretty(&workspaces)?;
    std::fs::write(path, contents)?;
    Ok(())
}

fn read_from_path(path: &PathBuf) -> anyhow::Result<Workspaces> {
    let contents = std::fs::read_to_string(path)?;
    let workspaces: Workspaces = serde_json::from_str(&contents)?;
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
        let mut space = Workspace::new("First Workspace", Uuid::new_v4());
        space.repo_paths.push("C:\\fake\\path0".to_owned());
        space.repo_paths.push("C:\\fake\\path1".to_owned());
        space.repo_paths.push("C:\\fake\\path2".to_owned());
        spaces.workspaces.insert(space.uuid,space);

        let mut space = Workspace::new("Second Workspace", Uuid::new_v4());
        space.repo_paths.push("C:\\fake\\path3".to_owned());
        space.repo_paths.push("C:\\fake\\path4".to_owned());
        space.repo_paths.push("C:\\fake\\path5".to_owned());
        spaces.workspaces.insert(space.uuid,space);
        spaces
    }

    #[test]
    fn should_get_list_of_workspaces() {
        let workspace_path = Workspaces::path();
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
        let loaded_spaces: Workspaces = serde_json::from_str(&serialized_spaces).unwrap();

        for (id, workspace) in spaces.workspaces.iter(){
            let loaded_workspace = &loaded_spaces.workspaces[&id];
            assert_eq!(workspace, loaded_workspace);
        }
    }

    #[test]
    fn should_persist_workspace() -> anyhow::Result<()> {
        let spaces = create_test_workspaces();
        let test_path = test_path();
        write_to_path(&test_path, &spaces)?;
        let loaded_spaces = read_from_path(&test_path)?;

        for (id, workspace) in spaces.workspaces.iter(){
            let loaded_workspace = &loaded_spaces.workspaces[&id];
            assert_eq!(workspace, loaded_workspace);
        }

        Ok(())
    }
}
