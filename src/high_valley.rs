use std::error::Error;
use thirtyfour::prelude::*;
use std::{thread, u16};
use std::time::Duration;
use futures::future::*;

static TARGET: &str = "https://www.highvalleytransit.org/bus-routes/";

#[derive(Debug)]
struct HVTStopTime {
    stop: String, 
    time: Vec<String>,
}

#[tokio::main]
async fn main() ->  Result<(), Box<dyn Error + Send + Sync>> {
    
    //initialize geckodriver (download at https://github.com/mozilla/geckodriver/releases)
    let mut caps = DesiredCapabilities::firefox();
    caps.add_firefox_arg("--enable-automation")?;
    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    //go to link and scrape out bus line subpage links
    driver.goto::<&str>(TARGET.as_ref()).await?;
    let busses = driver.find_all(By::Css("a[class='sqs-block-button-element--large sqs-button-element--secondary sqs-block-button-element']")).await?;
    let busses = try_join_all(busses.iter().map(|c| c.attr("href"))).await?;
    
    //iterate through each bus line subpage to collect the stops and times (bus arrival), storing them in timetable element
    let mut timetable: Vec<HVTStopTime>= Vec::new();
    for bus in busses {
        let link = format!("https://www.highvalleytransit.org{}", bus.unwrap());
        driver.goto::<&str>(link.as_ref()).await?;
        thread::sleep(Duration::new(1,0));    
        //need to get rid of this for loop
        let stops = driver.find_all(By::XPath("/html/body/div[8]/main/article/section[5]/div[2]/div/div/div/div[2]/div/div/table/thead/tr/th")).await?;
        let mut n: u16 = 1;
        for stop in stops {  
            let stop: String = stop.text().await?;
            let path = format!("/html/body/div[8]/main/article/section[5]/div[2]/div/div/div/div[2]/div/div/table/tbody/tr/td[{}]", n);
            let time = driver.find_all(By::XPath(&path)).await?;
            let time = try_join_all(time.iter().map(|c| c.text())).await?;
            timetable.push(HVTStopTime{stop, time});
            n = n+1;
        }
    }

    driver.quit().await?;
    //println!("testing {:?} and {:?}", timetable[1].stop, timetable[1].times[0]);
    println!("return vals: {:?}", timetable);

    Ok(())
    
}