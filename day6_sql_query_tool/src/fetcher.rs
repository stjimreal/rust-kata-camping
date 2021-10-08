/*
 * @Date: 2021-10-08 11:32:50
 * @LastEditors: LIULIJING
 * @LastEditTime: 2021-10-08 18:34:46
 */
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use tokio::fs;

// Rust 的 async_trait 还没有稳定，可以用 async_trait 宏
#[async_trait]
pub trait Fetch {
    type Error;
    async fn fetch(&self) -> Result<String, Self::Error>;
}

/// 从文件源或者 http 源中获取数据，组成 data frame
pub async fn retrieve_data(source: impl AsRef<str>) -> Result<String> {
    let name = source.as_ref();
    match &name[..4] {
        // 包括 http / https
        "http" => ProxyUrlFetcher(name).fetch().await,
        // 处理 file://<filename>
        "file" => FileFetcher(name).fetch().await,
        _=> return Err(anyhow!("We only support http/https/file at the moment"))
    }
}

struct UrlFetcher<'a>(pub(crate) &'a str);

struct ProxyUrlFetcher<'a>(pub(crate) &'a str);

struct FileFetcher<'a>(pub(crate) &'a str);

#[async_trait]
impl<'a> Fetch for UrlFetcher<'a> {
    type Error = anyhow::Error;

    async fn fetch(&self) -> Result<String, Self::Error>{
        Ok(reqwest::get(self.0).await?.text().await?)
    }
}

#[async_trait]
impl<'a> Fetch for ProxyUrlFetcher<'a> {
    type Error = anyhow::Error;

    async fn fetch(&self) -> Result<String, Self::Error>{
        let client = reqwest::Client::builder().proxy(reqwest::Proxy::all("http://localhost:7890")?).build()?;
        let data = client.get(self.0).send().await?.text().await?;
        // Ok(reqwest::get(self.0).await?.text().await?)
        Ok(data)
    }
}

#[async_trait]
impl<'a> Fetch for FileFetcher<'a> {
    type Error = anyhow::Error;

    async fn fetch(&self) -> Result<String, Self::Error> {
        Ok(fs::read_to_string(&self.0[7..]).await?)
    }
}