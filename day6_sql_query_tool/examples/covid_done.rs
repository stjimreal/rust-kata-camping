/*
 * @Date: 2021-10-08 18:51:06
 * @LastEditors: LIULIJING
 * @LastEditTime: 2021-10-08 18:54:55
 */

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // 使用 sql 从 URL 获取数据
    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    let sql = format!(
        "SELECT location AS name, total_cases, new_cases, total_deaths, new_deaths \
        FROM {} where new_deaths >= 500 ORDER BY new_cases DESC",
        url
    );
    let df1 = queryer::query(sql).await?;
    println!("{:?}", df1);
    Ok(())
}