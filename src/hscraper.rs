use playwright::Playwright;
use std::time::Duration;
use tokio::time::sleep;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Holiday {
    pub event: String,
    pub date: String,
    pub day: String,
}

pub struct HScraper {
    pub year: u32,
}

impl HScraper {
    pub fn new(year: u32) -> Self {
        HScraper { year }
    }

    #[allow(unused)]
    pub async fn get_holidays(&self) -> Result<Vec<Holiday>, Box<dyn std::error::Error>> {
        let url = format!(
            "https://www.officialgazette.gov.ph/nationwide-holidays/{}/",
            self.year
        );

        let playwright = Playwright::initialize().await?;
        playwright.prepare()?;

        let chromium = playwright.chromium();
        let browser = chromium.launcher()
            .headless(true)
            .launch()
            .await?;

        let context = browser.context_builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .build()
            .await?;

        let page = context.new_page().await?;

        page.goto_builder(&url).goto().await?;

        println!("Waiting for page to load...");
        sleep(Duration::from_secs(8)).await;

        let title = page.title().await?;
        println!("Page title: {}", title);

        // Extract holidays from the table
        let holidays_json = page.eval::<String>(
            r#"
            JSON.stringify(
                Array.from(document.querySelectorAll('tr.holiday-group')).map(row => {
                    const eventEl = row.querySelector('.holiday-what');
                    const dateEl = row.querySelector('.holidate abbr');
                    const dayEl = row.querySelector('.dayname');

                    return {
                        event: eventEl ? eventEl.textContent.trim() : '',
                        date: dateEl ? dateEl.getAttribute('title') : '',
                        day: dayEl ? dayEl.textContent.trim().replace(/[()]/g, '') : ''
                    };
                })
            )
            "#
        ).await?;

        let holidays: Vec<Holiday> = serde_json::from_str(&holidays_json)?;

        println!("\nHolidays found:");
        for holiday in &holidays {
            println!("{} - {} {}", holiday.event, holiday.date, holiday.day);
        }

        browser.close().await?;

        Ok(holidays)
    }

    pub async fn save_snapshot(&self) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!(
            "https://www.officialgazette.gov.ph/nationwide-holidays/{}/",
            self.year
        );

        let playwright = Playwright::initialize().await?;
        playwright.prepare()?;

        let chromium = playwright.chromium();
        let browser = chromium.launcher()
            .headless(true)
            .launch()
            .await?;

        let context = browser.context_builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .build()
            .await?;

        let page = context.new_page().await?;

        page.goto_builder(&url).goto().await?;

        println!("Waiting for page to load...");
        sleep(Duration::from_secs(8)).await;

        let html = page.content().await?;

        let snapshot_dir = "snapshots";
        if !Path::new(snapshot_dir).exists() {
            fs::create_dir(snapshot_dir)?;
        }

        let filename = format!("{}/holidays_{}.html", snapshot_dir, self.year);
        fs::write(&filename, &html)?;
        println!("HTML snapshot saved to: {}", filename);

        browser.close().await?;

        Ok(html)
    }

    pub fn load_snapshot(&self) -> Result<String, std::io::Error> {
        let filename = format!("snapshots/holidays_{}.html", self.year);
        fs::read_to_string(&filename)
    }

    pub fn snapshot_exists(&self) -> bool {
        let filename = format!("snapshots/holidays_{}.html", self.year);
        Path::new(&filename).exists()
    }

    pub fn parse_holidays_from_html(&self, html: &str) -> Result<Vec<Holiday>, Box<dyn std::error::Error>> {
        use scraper::{Html, Selector};

        let document = Html::parse_document(html);
        let row_selector = Selector::parse("tr.holiday-group").unwrap();
        let event_selector = Selector::parse(".holiday-what").unwrap();
        let date_selector = Selector::parse(".holidate abbr").unwrap();
        let day_selector = Selector::parse(".dayname").unwrap();

        let mut holidays = Vec::new();

        for row in document.select(&row_selector) {
            let event = row.select(&event_selector)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            let date = row.select(&date_selector)
                .next()
                .and_then(|el| el.value().attr("title"))
                .unwrap_or("")
                .to_string();

            let day = row.select(&day_selector)
                .next()
                .map(|el| el.text().collect::<String>().trim().replace(&['(', ')'][..], ""))
                .unwrap_or_default();

            if !event.is_empty() {
                holidays.push(Holiday { event, date, day });
            }
        }

        Ok(holidays)
    }

    pub fn delete_snapshot(&self) -> Result<(), std::io::Error> {
        let filename = format!("snapshots/holidays_{}.html", self.year);
        fs::remove_file(&filename)?;
        Ok(())
    }
}