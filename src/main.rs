use std::{
    fs,
    process::{Command, Stdio},
};
use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

#[tokio::main]
// async fn main() -> WebDriverResult<()> {
async fn main() {
    // let caps = DesiredCapabilities::chrome();
    // println!("Loading web page...");

    // let driver = WebDriver::new("http://localhost:9515", caps).await?;
    // println!("Loading web page...");

    // driver.goto("https://spacex.com").await?;
    // sleep(Duration::from_millis(3000)).await;
    // println!("Loaded web page!");

    // let sections = driver
    //     .find_all(By::Css("body > #wrapper > #scroller > #tiles > .section"))
    //     .await?;
    // println!("Sections found: {:?}", sections.len());

    // for section in sections {
    //     if let Ok(img) = section.find(By::Css("span")).await {
    //         if let Ok(title_elem) = section.find(By::Css(".inner-left-bottom > h2")).await {
    //             if let Ok(title) = title_elem.text().await {
    //                 if let Some(url) = img
    //                     .attr("data-desktop")
    //                     .await
    //                     .expect("Failed to get url of image")
    //                 {
    //                     println!("Url: {}", url);
    //                     println!("Title: {}", title);

    //                     let output = Command::new("ascii-image-converter")
    //                         .args(&["falcon9.jpg", "-b", "--threshold", "30", "-C"])
    //                         .stdout(Stdio::piped())
    //                         .output()
    //                         .unwrap();

    //                     let stdout = String::from_utf8(output.stdout).unwrap();

    //                     println!("{}", stdout);
    //                     // if output.status.success() {
    //                     //     let ascii_art = String::from_utf8_lossy(&output.stdout);
    //                     //     println!("{}", ascii_art.to_string());
    //                     // } else {
    //                     //     eprintln!(
    //                     //         "ASCII Image Conversion Failed: {}",
    //                     //         String::from_utf8_lossy(&output.stderr)
    //                     //     );
    //                     // }

    //                     if let Ok(info_elem) = section.find(By::Css(".inner-left-bottom > p")).await
    //                     {
    //                         if let Ok(info_text) = info_elem.text().await {
    //                             println!("Info: {}\n", info_text);
    //                         }
    //                     }
    //                 } else {
    //                     eprintln!("No url found for image");
    //                 }
    //             } else {
    //                 eprintln!("Failed to read title text");
    //             }
    //         } else {
    //             eprintln!("Failed to read title element");
    //         }
    //     } else {
    //         eprintln!("Failed to read image");
    //     }
    // }

    // driver.quit().await?;
    // Ok(())
    let output = Command::new("cmd")
        .arg("/C")
        .arg("ascii-image-converter falcon9.jpg -b --threshold 30 -C")
        // .args([
        //     "/C",
        //     "ascii-image-converter falcon9.jpg -b --threshold 30 -C",
        //     // "falcon9.jpg",
        //     // "-b",
        //     // "--threshold",
        //     // "30",
        //     // "-C",
        // ])
        // Tell the OS to record the command's output
        .stdout(Stdio::piped())
        // execute the command, wait for it to complete, then capture the output
        .output()
        // Blow up if the OS was unable to start the program
        .unwrap();

    // extract the raw bytes that we captured and interpret them as a string
    let stdout = String::from_utf8(output.stdout).unwrap();

    let art = fs::read_to_string("falcon9-ascii-art.txt").unwrap();
    println!("{}", art);
}
