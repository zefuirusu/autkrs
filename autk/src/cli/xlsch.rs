use calamine::{open_workbook, DataType, Reader, Xlsx};
use comfy_table::{Cell, Table};
use rayon::prelude::*;
use regex::Regex;
use std::path::PathBuf;
 
/// 搜索Excel表格中指定列的正则匹配结果 
pub fn re_match_col(
  regex_str:String,
  shtna:String,
  col_index:usize,
  title:usize,
  ifp:String,
)->(){
  let re=Regex::new(&regex_str).unwrap();
  let mut wb:Xlsx<_>=open_workbook(PathBuf::from(ifp.as_str())).expect("Cannot open the target Excel!");
  let range=wb.worksheet_range(&shtna).expect("This sheet does not exist!");
  let matches:Vec<_>=range.expect("Fail to get range!")
    .rows()
    .par_bridge()
    .filter_map(
      |row|{
        let cell=row.get(col_index as usize -1).expect("Column index out of range.");
        let text=match cell{
          DataType::String(s) => s,
          DataType::Float(f) => &f.to_string(),
          DataType::Int(i) => &i.to_string(),
          DataType::Bool(b) => if *b { "true" } else { "false" },
          DataType::DateTime(f) => &f.to_string(),
          DataType::Empty => "",
          _ => "",
        };
        if re.is_match(text){
          Some(text.to_string())
        }else{
          Option::None
        }
      }
    ).collect();
  let mut table=Table::new();
  table.set_header(
    vec![
      Cell::new("No.").add_attribute(comfy_table::Attribute::Bold),
      Cell::new("Match Result").add_attribute(comfy_table::Attribute::Bold),
    ]
  );
  for (i,matched) in matches.iter().enumerate(){
    table.add_row(
      vec![(i+1).to_string(),matched.clone()]
    );
  }
  println!("{}",table);
}
