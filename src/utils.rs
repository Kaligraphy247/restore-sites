/// Opens a site in a private window for different OSes.
///
/// # Arguments
///
/// * `site` - A string representing the site to open
pub fn run_firefox_command(site: &str) {
    if cfg!(target_os = "windows") {
        println!("Running On Windows, Nothing to do at this time");
    } else if cfg!(target_os = "macos") {
        println!("Running On MacOS, Nothing to do at this time");
    } else if cfg!(target_os = "linux") {
        std::process::Command::new("firefox")
            .args(["--private-window", site])
            .spawn()
            .expect("firefox failed to start");
    }
}

/// Open sites using specified browser via the open_browser_fn
///
/// # Arguments
///
/// * `sites` - A vector of strings representing the sites to open
/// * `open_browser_fn` - A function that takes a string representing a site and opens it in the browser
pub fn open_sites(sites: Vec<String>, open_browser_fn: fn(site: &str)) {
    let pb = indicatif::ProgressBar::new(100);
    let mut count = 0;

    for site in &sites {
        count += 1;
        pb.println(format!("Opening {} ...", site));
        open_browser_fn(site);
        pb.set_position(((count * 100) / sites.len()) as u64);
        std::thread::sleep(std::time::Duration::from_millis(350));
    }

    pb.finish_and_clear();
    println!("Restored {} sites.\nDone!", sites.len());
}

/// Opens a list of sites from a file using the specified browser.
///
/// # Arguments
///
/// * `sites` - The path to the file containing the list of sites to open.
/// * `open_browser_fn` - The function to use to open each site in the browser.
pub fn open_sites_from_file(sites: std::path::PathBuf, open_browser_fn: fn(site: &str)) {
    let pb = indicatif::ProgressBar::new(100);
    let file_exists = std::path::PathBuf::from(sites.clone()).exists();
    if file_exists {
        let content = std::fs::read_to_string(&sites).expect("Failed to read file");
        let content: Vec<&str> = content.lines().collect();
        let mut count = 0;
        for site in &content {
            count += 1;
            pb.println(format!("Opening {} ...", site));
            open_browser_fn(site);
            pb.set_position(((count * 100) / content.len()) as u64);
            std::thread::sleep(std::time::Duration::from_millis(350));
        }
        pb.finish_and_clear();
        println!("Restored {} sites.\nDone!", content.len());
    } else {
        eprintln!("File {} does not exist!", sites.to_str().unwrap());
    }
}
