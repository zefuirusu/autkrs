use std::path::PathBuf;
use calamine::{open_workbook,Reader,Xlsx,Xls,DataType};
// use polars::prelude::*;

use crate::calculation::XlSheet;
use crate::mapper::XlMap;
use crate::meta::ShtMeta;


impl XlMap<'_> for XlSheet<'_>{
}
impl XlSheet<'_>{
    pub fn load_raw_data()->(){}
}

// load Excel data as DataFrame;
pub fn xl2df<'load>(
  shtmeta:&'load ShtMeta,
)
// ->PolarsResult<DataFrame>
{
  // let wb:Xlsx<_>=match open_workbook(PathBuf::from(path)){
    // todo!()
  // };
  todo!()
}
