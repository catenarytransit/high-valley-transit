use std::error::Error;
use thirtyfour::*;
use std::thread;
use std::time::Duration;

static TARGET: &str = "https://metrolinktrains.com/rtt/StationScheduleList.json";

#[tokio::main]
async fn main() ->  Result<(), Box<dyn Error + Send + Sync>> {
    
    let mut caps = DesiredCapabilities::firefox();
    caps.add_firefox_arg("--enable-automation")?;
    let driver = WebDriver::new("http://localhost:4444", caps).await?;
    
    driver.goto::<&str>(TARGET.as_ref()).await?;
    thread::sleep(Duration::new(5,0));  
    
    let mut list = Vec::new();
    let elems = driver.find_all(By::XPath("/html/body/div/div/div/div[1]/div/div/div[2]/table/tbody/tr/td/span/span")).await?;
    for elem in elems{
        let property_value: String = elem.text().await?;
        list.push(property_value);
    }
 
    println!("return vals: {:?}", list);


    driver.quit().await?;

    Ok(())
    
}