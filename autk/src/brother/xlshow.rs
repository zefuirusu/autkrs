use std::path::PathBuf;
use calamine::{open_workbook, DataType, Reader, Xlsx};
use rayon::prelude::*;

use crate::brother::cell_value2string;

pub fn get_row(
  row_index:usize,// starts from 1;
  ifp:String,
  shtna:String,
)->Vec<String>{
  let mut wb:Xlsx<_>= open_workbook(PathBuf::from(ifp.as_str()))
    .expect("Cannot open the target Excel!");
  wb.worksheet_range(&shtna.as_str())
    .expect("This sheet does not exist!")
    .expect("Fail to get range!")
    .rows()
    .nth(row_index-1)
    .expect("Fail to the this row!")
    .into_par_iter()
    .map(
      |cell_value|{
        cell_value2string(cell_value)
      }
    )
    // .into_iter()
    .collect()
}
pub fn get_rows(
  rows_index:Vec<usize>,
  ifp:String,
  shtna:String,
)->Vec<Vec<String>>{
  rows_index
    .into_par_iter()
    .map(
      |row_index|{
        get_row(row_index,ifp.clone(),shtna.clone(),)
      }
    ).collect()
}
pub fn get_col(
  col_index:usize
)->Vec<String>{
  todo!()
}
