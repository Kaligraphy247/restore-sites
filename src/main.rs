use clap::Parser;
mod utils;
use utils::{open_sites, open_sites_from_file, run_firefox_command};

/// Restore previously visited sites into an Incognito/Private mode
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct RestoreSitesCLI {
    /// browser, i.e. chrome, firefox
    #[arg(short, long)]
    browser: String,

    /// Configuration file, i.e. config.toml
    #[arg(short, long, default_value = None, required = false)]
    config: Option<std::path::PathBuf>,

    /// site, i.e. https://google.com
    #[arg(short, long, value_delimiter = ' ', num_args=1.., required = false)]
    site: Option<Vec<String>>,

    /// sites, a file containing a list of sites
    #[arg(long, default_value = None)]
    sites: Option<std::path::PathBuf>,
}

fn main() {
    let args = RestoreSitesCLI::parse();
    let browser = args.browser;

    match browser.as_str() {
        "chrome" => {
            println!("Running On Chrome, Not Supported at this time");
        }
        "firefox" => {
            if let Some(sites) = args.site {
                open_sites(sites, run_firefox_command);
            } else if let Some(sites) = args.sites {
                open_sites_from_file(sites, run_firefox_command);
            }
        }
        _ => {
            eprintln!("Browser {} is not supported at this time.", browser);
            std::process::exit(1);
        }
    }
}
