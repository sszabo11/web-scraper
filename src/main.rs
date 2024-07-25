use std::{thread, time::Duration};

use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    println!("Loading web page...");
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    println!("Loading web page...");
    driver.goto("https://spacex.com").await?;

    thread::sleep(Duration::from_millis(2000));
    println!("Loaded web page!");

    let sections = driver
        .find_all(By::Css("body > #wrapper > #scroller > #tiles > .section"))
        .await?;
    println!("{:?}", sections);

    for section in sections {
        let img = section
            .find(By::Css("span"))
            .await
            .expect("Failed to read image");

        let url = img
            .attr("data-desktop")
            .await
            .expect("Failed to get url of image");
        println!("{}", url.clone().unwrap());

        if let Some(url) = url {
            println!("Url: {}", url)
        } else {
            eprintln!("No url found")
        };
    }

    driver.quit().await?;
    Ok(())
}
