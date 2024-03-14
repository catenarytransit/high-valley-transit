use std::error::Error;
use thirtyfour::prelude::*;
use std::{thread, u16};
use std::time::Duration;
use futures::future::*;

static TARGET: &str = "https://www.highvalleytransit.org/bus-101-to-deer-valley";

#[derive(Debug)]
struct HVTStopTime {
    stop: String, 
    times: Vec<String>,
}

#[tokio::main]
async fn main() ->  Result<(), Box<dyn Error + Send + Sync>> {
    let mut caps = DesiredCapabilities::firefox();
    caps.add_firefox_arg("--enable-automation")?;
    let driver = WebDriver::new("http://localhost:4444", caps).await?;
    
    driver.goto::<&str>(TARGET.as_ref()).await?;
    thread::sleep(Duration::new(5,0));  
    
    let mut timetable: Vec<HVTStopTime>= Vec::new(); 
    let titles = driver.find_all(By::XPath("/html/body/div[8]/main/article/section[5]/div[2]/div/div/div/div[2]/div/div/table/thead/tr/th")).await?;
    let mut n: u16 = 0;
    for title in titles {
        let stop: String = title.text().await?;
        let path = format!("/html/body/div[8]/main/article/section[5]/div[2]/div/div/div/div[2]/div/div/table/tbody/tr/td[{}]", n);
        let times = driver.find_all(By::XPath(&path)).await?;
        let times = try_join_all(times.iter().map(|c| c.text())).await?;
        timetable.push(HVTStopTime{stop, times})
    }

    driver.quit().await?;

    println!("return vals: {:?}", timetable);

    Ok(())
    
}