use std::path::PathBuf;
use calamine::{open_workbook, DataType, Reader, Xlsx};
use rayon::prelude::*;
use regex::Regex;
use comfy_table::{Cell, Table};

use crate::meta::MatchMeta;
use crate::cli::evince::{term_show_table,term_show_rows};
use crate::brother::cell_value2string;
use crate::brother::xlshow::{get_row,get_rows};

/// match specific column in Excel by regex;
pub fn re_match_col(
  regex_str:String,
  match_col:usize,
  shtna:String,
  ifp:String,
)->Vec<Vec<String>>{
  let regex_item=Regex::new(&regex_str).unwrap();
  let mut wb:Xlsx<_>=open_workbook(PathBuf::from(ifp.as_str())).expect("Cannot open the target Excel!");
  let range=wb.worksheet_range(&shtna).expect("This sheet does not exist!");
  let matches:Vec<_>=range.expect("Fail to get range!")
    .rows()
    .par_bridge()
    .filter_map(
      |row|{
        let match_cell=row.get(match_col as usize -1).expect("Match column index out of range.");
        let text_match_col=cell_value2string(&match_cell);
        if regex_item.is_match(&text_match_col){
          // let resu_cell=if resu_col==0usize {
            // row.get(match_col as usize -1).expect("Result column index out of range!")
          // }else{
            // row.get(resu_col as usize -1).expect("Result column index out of range!")
          // };
          // let text_resu_col=cell_value2string(&resu_cell);
          // Some(text_match_col.to_string())
          // Some(text_resu_col.to_string())
          Some(
            row.into_iter().map(|cell|{cell_value2string(cell)}).collect::<Vec<_>>()
          )
        }else{
          Option::None
        }
      }
    ).collect();
  let mut table=Table::new();
  /*
  table.set_header(
    vec![
      Cell::new("No.").add_attribute(comfy_table::Attribute::Bold),
      Cell::new("Result Column").add_attribute(comfy_table::Attribute::Bold),
    ]
  );
  for (i,matched) in matches.iter().enumerate(){
    table.add_row(
      vec![(i+1).to_string(),matched.clone()]
    );
  }
  println!("{}",table);
  matches
  */
  // table.set_header(get_row(title,ifp,shtna));
  // for matched in matches.iter(){
    // table.add_row(matches);
  // }
  // println!("{}",table);
  matches
}
pub fn par_re_match_col(
  regex_str:String,
  meta:MatchMeta,
)->Vec<String>{
  todo!()
}
#[test]
fn test_match()->(){
}
