use anyhow::bail;
use serde::{Deserialize, Serialize};

use crate::paths::sourcetree_settings_path;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename = "ArrayOfStrings")]
pub struct OpenTabs {
    #[serde(rename = "string")]
    pub tabs: Vec<String>,
}

impl OpenTabs {
    pub fn path() -> Option<PathBuf> {
        let Some(settings_path) = sourcetree_settings_path() else { 
            return None; 
        };

        Some(settings_path.join("opentabs.xml"))
    }

    pub fn write(open_tabs: &OpenTabs) -> anyhow::Result<()> {
        let Some(path) = OpenTabs::path() else { 
            bail!("Error getting open tabs file path for reading."); 
        };

        write_to_path(&path, open_tabs)?;
        Ok(())
    }

    pub fn read() -> anyhow::Result<OpenTabs> {
        let Some(path) = OpenTabs::path() else { 
            bail!("Error getting open tabs file path for reading."); 
        };

        let open_tabs = read_from_path(&path)?;
        Ok(open_tabs)
    }
}

fn write_to_path(path: &PathBuf, open_tabs: &OpenTabs) -> anyhow::Result<()> {
    let contents = serde_xml_rs::to_string(&open_tabs)?;
    std::fs::write(path, contents)?;
    Ok(())
}

fn read_from_path(path: &PathBuf) -> anyhow::Result<OpenTabs> {
    let contents = std::fs::read_to_string(path)?;
    let open_tabs: OpenTabs = serde_xml_rs::from_str(&contents)?;
    Ok(open_tabs)
}

#[cfg(test)]
mod tests {
    use serde_xml_rs::from_str;

    use super::*;

    fn test_path() -> PathBuf {
        let path = sourcetree_settings_path().unwrap();
        path.join("opentabs-test.xml")
    }

    fn create_test_open_tabs() -> OpenTabs {
        let mut open_tabs = OpenTabs::default();
        open_tabs.tabs.push(r#"C:\example\project-one"#.to_owned());
        open_tabs.tabs.push(r#"C:\example\project-two"#.to_owned());
        open_tabs
    }

    #[test]
    fn should_load_open_tabs() {
        let tab_doc = r#"<?xml version="1.0"?>
                        <ArrayOfString>
                            <string>C:\example\project-one</string>
                            <string>C:\example\project-two</string>
                        </ArrayOfString>"#;

        let open_tabs: OpenTabs = from_str(tab_doc).unwrap();
        assert_eq!(open_tabs.tabs[0], r#"C:\example\project-one"#);
        assert_eq!(open_tabs.tabs[1], r#"C:\example\project-two"#);
    }

    #[test]
    fn should_persist_open_tabs() -> anyhow::Result<()> {
        let open_tabs = create_test_open_tabs();
        let test_path = test_path();
        write_to_path(&test_path, &open_tabs)?;
        let open_tabs = read_from_path(&test_path)?;
        assert_eq!(open_tabs.tabs[0], r#"C:\example\project-one"#);
        assert_eq!(open_tabs.tabs[1], r#"C:\example\project-two"#);
        Ok(())
    }
}
