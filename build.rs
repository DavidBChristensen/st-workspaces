extern crate winres;

fn main() {
    // Check if we're compiling for Windows
    //if cfg!(target_os = "windows") {
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/st-workspaces.ico");
    res.compile().unwrap();
    //}
}
