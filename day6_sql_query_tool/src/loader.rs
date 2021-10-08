/*
 * @Date: 2021-10-08 17:36:28
 * @LastEditors: LIULIJING
 * @LastEditTime: 2021-10-08 17:51:07
 */

use std::io::Cursor;

use crate::DataSet;
use anyhow::Result;
use polars::{io::SerReader, prelude::CsvReader};

pub trait Load {
    type Error;
    fn load(self) -> Result<DataSet, Self::Error>;
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Loader {
    Csv(CsvLoader),
}

#[derive(Debug, Default)]
pub struct CsvLoader(pub(crate) String);

impl Loader {
    pub fn load(self) -> Result<DataSet>{
        match self {
            Loader::Csv(csv) => csv.load(),
        }
    }
}

pub fn detect_content(data: String) -> Loader {
    // TODO: 内容检测
    Loader::Csv(CsvLoader(data))
}

impl Load for CsvLoader {
    type Error = anyhow::Error;

    fn load(self) -> Result<DataSet, Self::Error> {
        let df = CsvReader::new(Cursor::new(self.0))
                                    .infer_schema(Some(16))
                                    .finish()?;
        Ok(DataSet(df))
    }
}