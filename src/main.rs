use clap::Parser;
use json::object;
use thirtyfour::prelude::*;
use std::collections::{HashMap, HashSet};
use std::process::Command;
use std::io::Write; 

#[derive(Parser, Debug)] #[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,
    
    #[arg(short, long = "start", default_value_t = 1)]
    start_page: u32,

    #[arg(short, long = "end", default_value_t = 1)]
    end_page: u32,

    #[arg(short, long = "csv", default_value = "")]
    csv_filename: String,

    #[arg(short, long = "json", default_value = "")]
    json_filename: String,
}

struct Reaper {
    driver: WebDriver,
}

impl Reaper {
    fn new(driver: WebDriver) -> Self {
        Self {
            driver,
        }
    }

    async fn connect_to_page(&self, url: &str, page: u32) -> WebDriverResult<()> {
        self.driver.goto(url).await?;
        if page > 1 {
            let href = self.driver.find(By::Css(format!("a[aria-label='Page {page}']"))).await?.attr("href").await?.unwrap();
            self.driver.goto(format!("https://www.google.com{href}")).await?;
        }
        Ok(())
    }

    async fn reap_links(&self) -> Result<Vec<String>, WebDriverError> {
        let mut links_vec = vec![];
        let links = self.driver.find_all(By::Tag("a")).await?;

        for link in &links {
            match link.attr("href").await? {
                Some(href) => {
                    if !href.starts_with("/") &&
                        !href.starts_with("#") &&
                            !href.starts_with("https://support.google") &&
                            !href.starts_with("https://www.google") &&
                            !href.starts_with("https://accounts.google") &&
                            !href.starts_with("https://policies.google") {

                                links_vec.push(href);
                            }
                }
                None => continue,   
            }
        }

        return Ok(links_vec);
    }

    async fn reap_emails(&self, link: &str) -> Result<HashSet<String>, WebDriverError> {
        let response: reqwest::Response; 
        match reqwest::get(link).await {
            Ok(res) => response = res,
            Err(_) => return Ok(HashSet::new()),
        }
        let html_content = response.text().await?;
        let re = regex::Regex::new(r"([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})").unwrap();
        let mut emails = HashSet::new();
        for caps in re.captures_iter(&html_content) {
            emails.insert(caps[0].to_string());
        }
        Ok(emails)
    }
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    start_webdriver();
    let mut caps = DesiredCapabilities::chrome();
    let _ = caps.add_arg("--headless");
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    let args = Args::parse();
    let reaper = Reaper::new(driver.clone());    

    let mut links = vec![];
    println!("\nParsing links from pages {}-{}....", args.start_page, args.end_page);
    for page in args.start_page..args.end_page+1 {
        reaper.connect_to_page(&args.url, page).await?;
        reaper.reap_links().await?.iter().for_each(|link| {
            links.push(link.clone());
        });
    }

    driver.quit().await?;
    
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();

    for (i, link) in links.iter().enumerate() {
        println!("({}/{}) Parsing emails from {}...", i + 1, links.len(), link);

        let mut emails = reaper.reap_emails(link).await?;
        reaper.reap_emails(&format!("{link}/contact")).await?.iter().for_each(|email| {
            emails.insert(email.clone());
        });
        reaper.reap_emails(&format!("{link}/contacts")).await?.iter().for_each(|email| {
            emails.insert(email.clone());
        });
        map.insert(link.to_string(), emails);
    }

    if args.csv_filename != "" {
        write_to_csv(map.clone(), args.csv_filename).unwrap();
    } 
    if args.json_filename != "" {
        write_to_json(map.clone(), args.json_filename).unwrap();
    }
    else {
        for (_, email_set) in map {
            for email in email_set {
                println!("{}", email);
            }
        }   
    }

    Ok(())
}

fn write_to_csv(map: HashMap<String, HashSet<String>>, filename: String) -> Result<(), csv::Error> {
    let mut writer = csv::Writer::from_path(filename)?;
    for (link, email_set) in map {
        writer.write_record(&[
                            link,
                            email_set.iter().nth(0).unwrap_or(&"".to_string()).to_string(),
                            email_set.iter().nth(1).unwrap_or(&"".to_string()).to_string(),
                            email_set.iter().nth(2).unwrap_or(&"".to_string()).to_string()
        ])?;
        writer.flush()?;
    }
    Ok(())
}

fn write_to_json(map: HashMap<String, HashSet<String>>, filename: String) -> Result<(), std::io::Error> {
    let mut file = std::fs::File::create(filename)?;
    file.write_all(b"[")?;
    let mut count = 0;
    let length = map.len();
    for (link, email_set) in map {
        count += 1;
        let json = object! {
            link: link,
            emails: email_set.iter().map(|x| x.to_string()).collect::<Vec<String>>()
        };

        file.write_all(json.dump().as_bytes())?;
        if count != length {
            file.write_all(b",")?;
        }
    }
    file.write_all(b"]")?;
    Ok(())
}

fn start_webdriver() {
   Command::new("chromedriver")
       .spawn()
       .expect("process failed to execute"); 
}
