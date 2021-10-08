/*
 * @Date: 2021-10-08 18:40:17
 * @LastEditors: LIULIJING
 * @LastEditTime: 2021-10-08 18:50:05
 */

use anyhow::Result;
#[tokio::main]
async fn main() -> Result<()> {
    let url = "file:///Users/jimlau/MOYD_cloud.csv";
    let sql = format!("SELECT file_name, cloud_ratio FROM {} WHERE cloud_ratio>=50 \
        ORDER BY cloud_ratio DESC", url);
    let res = queryer::query(sql).await?;
    println!("{:?}", res);
    Ok(())
}