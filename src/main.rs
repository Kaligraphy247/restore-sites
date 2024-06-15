use clap::Parser;

fn run_firefox_command(site: &str) {
    if cfg!(target_os = "windows") {
        println!("Running On Windows, Doing Nothing at this time");
    } else if cfg!(target_os = "macos") {
        println!("Running On MacOS, Doing Nothing at this time");
    } else if cfg!(target_os = "linux") {
        std::process::Command::new("firefox")
            .args(["--private-window", site])
            .spawn()
            .expect("firefox failed to start");
    }
}

/// Restore previously visited sites into an Incognito/Private mode
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// browser, i.e. chrome, firefox
    #[arg(short, long)]
    browser: String,

    /// Configuration file, i.e. config.toml
    #[arg(short, long, default_value = None, required = false)]
    config: Option<std::path::PathBuf>,

    /// site, i.e. https://google.com
    #[arg(short, long)]
    site: Vec<String>,

    /// sites, a file containing a list of sites
    #[arg(long, default_value = None)]
    sites: Option<std::path::PathBuf>,
}

fn main() {
    let args = Args::parse();
    // println!("{args:?}");

    let pb = indicatif::ProgressBar::new(100);

    if let Some(sites) = args.sites {
        let file_exist = std::path::PathBuf::from(sites.clone()).exists();
        if file_exist {
            let content = std::fs::read_to_string(&sites).expect("Failed to read file");
            let content: Vec<&str> = content.lines().collect();
            // println!("{content:?}"); 
            let mut count = 0;
            for site in &content {
                count += 1;
                pb.println(format!("Opening {} ...", site));
                run_firefox_command(&site);
                pb.set_position(((count * 100) / content.len()) as u64);
                std::thread::sleep(std::time::Duration::from_millis(350));
            }
            pb.finish_and_clear();
            println!("Restored {} sites.\nDone!", content.len());
        } else {
            eprintln!("File {:?} does not exist!", sites);
        }
    }
}
