# SourceTree Workspaces

Not affiliated with Atlassian.

## We Want to Save our Tabs in Workspaces

Often when working with SourceTree, you might find yourself opening multiple tabs for a project.  
You may be working on one task that you need to span multiple tabs; or, maybe just have the same 
repositories open for different projects.

Bookmarks get us part-way there, but it is a hassle to close all the other tabs, and open only the
ones you want. Especially if you are doing this multiple times in the day.

Other git clients have solved this with the idea of workspaces. SourceTree doesn't have this 
yet...so that is why this repository exists.

## The Short Roadmap

- Initial repository
- Launch the st-workspaces GUI.
- Save current tabs to new workspace.
    - from ui save button

-- We are here --

    - from custom action
- Load tabs from selected workspace.
- "Install" st-workspaces to custom actions.
- Manage workspaces
    - add
    - edit
    - delete

## What if Atlassian Adds the Workspaces Feature?

Awesome! Then there won't be a need for this any more. But until then, we want to stay sane.

## How Does it Work?

SourceTree encourages you to edit the settings files for custom actions and open tabs [Edit Settings Files](https://confluence.atlassian.com/sourcetreekb/edit-sourcetree-configurations-without-opening-the-application-windows-785323524.html).

This project simply edits these settings through the use of custom actions. [Custom Actions](https://confluence.atlassian.com/sourcetreekb/using-git-in-custom-actions-785323500.html)

## Version Compatibility

st-workspaces version -> SourceTree version

0.1.0 -> 3.4
