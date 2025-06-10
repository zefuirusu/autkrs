use std::path::PathBuf;
use calamine::{open_workbook, DataType, Reader, Xlsx};
use rayon::prelude::*;
use regex::Regex;
use comfy_table::{Cell, Table};

use crate::meta::MatchMeta;
use crate::cli::evince::{term_show_table,term_show_rows};
use crate::brother::xlshow::{get_row,get_rows};
 
pub fn cell_value2string(
  cell_value:&DataType,
)->String{
  match cell_value{
      DataType::String(s) => s.clone(),
      DataType::Float(f) => f.to_string(),
      DataType::Int(i) => i.to_string(),
      DataType::Bool(b) => if *b { "true".to_string() } else { "false".to_string() },
      DataType::DateTime(f) => f.to_string(),
      DataType::Empty => "".to_string(),
      DataType::Duration(d) => format!("{:?}", d),
      DataType::DateTimeIso(dt) => format!("{}", dt),
      DataType::DurationIso(d) => format!("{}", d),
      _ => "".to_string(),
  }
}
/// match specific column in Excel by regex;
pub fn re_match_col(
  regex_str:String,
  resu_col:usize,
  match_col:usize,
  shtna:String,
  title:usize,
  ifp:String,
)->Vec<String>{
  let regex_item=Regex::new(&regex_str).unwrap();
  let mut wb:Xlsx<_>=open_workbook(PathBuf::from(ifp.as_str())).expect("Cannot open the target Excel!");
  let range=wb.worksheet_range(&shtna).expect("This sheet does not exist!");
  let matches:Vec<_>=range.expect("Fail to get range!")
    .rows()
    .par_bridge()
    .filter_map(
      |row|{
        let match_cell=row.get(match_col as usize -1).expect("Match column index out of range.");
        // let text_match_col=match match_cell{
          // DataType::String(s) => s,
          // DataType::Float(f) => &f.to_string(),
          // DataType::Int(i) => &i.to_string(),
          // DataType::Bool(b) => if *b { "true" } else { "false" },
          // DataType::DateTime(f) => &f.to_string(),
          // DataType::Empty => "",
          // _ => "",
        // };
        let text_match_col=cell_value2string(&match_cell);
        if regex_item.is_match(&text_match_col){
          let resu_cell=if resu_col==0usize {
            row.get(match_col as usize -1).expect("Result column index out of range!")
          }else{
            row.get(resu_col as usize -1).expect("Result column index out of range!")
          };
          // let text_resu_col=match resu_cell{
            // DataType::String(s) => s,
            // DataType::Float(f) => &f.to_string(),
            // DataType::Int(i) => &i.to_string(),
            // DataType::Bool(b) => if *b { "true" } else { "false" },
            // DataType::DateTime(f) => &f.to_string(),
            // DataType::Empty => "",
            // _ => "",
          // };
          let text_resu_col=cell_value2string(&resu_cell);
          // Some(text_match_col.to_string())
          // TODO 这里可以对row进行collect();
          Some(text_resu_col.to_string())
        }else{
          Option::None
        }
      }
    ).collect();
    //// TODO:
    // term_show_table(
      // get_row(title,ifp,shtna,),
      // get_rows(,ifp,shtna),
    // )
  let mut table=Table::new();
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
}
pub fn par_re_match_col(
  regex_str:String,
  meta:MatchMeta,
)->Vec<String>{
  todo!()
}
