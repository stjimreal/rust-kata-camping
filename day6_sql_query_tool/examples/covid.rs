/*
 * @Date: 2021-10-04 00:30:03
 * @LastEditors: LIULIJING
 * @LastEditTime: 2021-10-08 18:55:36
 */
use anyhow::Result;
use polars::prelude::*;
use queryer::query;
use std::io::Cursor;

#[tokio::main]
async fn main() ->Result<()> {
    tracing_subscriber::fmt::init();

    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    // let url = "https://github.com/CSSEGISandData/COVID-19/blob/master/csse_covid_19_data/csse_covid_19_daily_reports_us/01-14-2021.csv";
    // let url = "https://github.com/stjimreal/";
    // let url = "http://baidu.com";

    // 使用 polars 直接请求
    let client = reqwest::Client::builder().proxy(reqwest::Proxy::all("http://localhost:7890")?).build()?;
    let data = client.get(url).send().await?.text().await?;
    // let data = reqwest::get(url).await?.text().await?;
    // let client = reqwest::Client::builder()?
    // .proxy(reqwest::Proxy::all("http://pro.xy")?)
    // .build()?;
    let df = CsvReader::new(Cursor::new(data))
        .infer_schema(Some(16))
        .finish()?;
    
    let filtered = df.filter(&df["new_deaths"].gt(500))?;
    println!(
        "{:?}",
        filtered.select((
            "location",
            "total_cases",
            "new_cases",
            "total_deaths",
            "new_deaths",
        ))
    );

    Ok(())
}