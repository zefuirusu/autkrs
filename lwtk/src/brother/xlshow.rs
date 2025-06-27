use std::path::PathBuf;
use std::collections::HashMap;
use calamine::{open_workbook, Reader,Xlsx};
use rayon::prelude::*;

use crate::brother::value2str;

pub fn get_shape(
  ifp:String,
)->HashMap<String,Vec<usize>>{
  //TODO: This function does not run at multi-thread by rayon currently;
  let mut shape:HashMap<String,Vec<usize>>=HashMap::new();
  let mut wb:Xlsx<_>=open_workbook(PathBuf::from(ifp.as_str()))
    .expect("Cannot open Excel! Invalid file path!");
  for shtna in wb.sheet_names(){
    let range:calamine::Range<_>=wb.worksheet_range(&shtna)
      .expect("Cannot get range!");
    shape.insert(
      shtna.to_string(),Vec::from([range.get_size().0,range.get_size().1])
    );
  }
  shape
}
pub fn get_row(
  _row_index:Option<usize>,// starts from 1;
  ifp:String,
  shtna:String,
)->Vec<String>{
  match _row_index{
    Some(row_index)=>{
      let mut wb:Xlsx<_>= open_workbook(PathBuf::from(ifp.as_str()))
        .expect("Cannot open the target Excel!");
      wb.worksheet_range(&shtna.as_str())
        .expect("Cannot get range!")
        .rows()
        .nth(row_index-1)
        .expect("Fail to the this row!")
        .into_par_iter()
        .map(
          |cell_value|{
            value2str(cell_value)
          }
        )
        // .into_iter()
        .collect()
    },
    None=>{
      Vec::new()
    },
  }
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
        get_row(Some(row_index),ifp.clone(),shtna.clone(),)
      }
    ).collect()
}
pub fn get_col(
  // col_index:usize
)->Vec<String>{
  todo!()
}
