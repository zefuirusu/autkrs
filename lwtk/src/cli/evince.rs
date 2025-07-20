use comfy_table::{Cell,Table,ContentArrangement};
use rayon::prelude::*;

pub fn term_show_rows(
  rows:Vec<Vec<String>>
)->()
{
  let mut table:Table=Table::new();
  for row in rows.into_iter(){
    table.add_row(row);
  }
  println!("{}",table);
}
pub fn term_show_table(
  title:Vec<String>,
  rows:Vec<Vec<String>>,
)
->()
{
  let mut table:Table=Table::new();
  table.set_content_arrangement(ContentArrangement::Dynamic);
  table.set_header(
    title.into_iter().map(
      |string_value|{Cell::new(string_value).add_attribute(comfy_table::Attribute::Bold)}
    )
  );
  for row in rows.into_iter(){
    table.add_row(row);
  }
  println!("{}",table);
}
fn row_from_multi_table(
  // TODO brother::xlshow::get_title_from_tables;
  lines:Vec<(String,(usize,usize))>,
  shtli:Vec<(String,String)>,
)->Vec<Vec<String>>{
  todo!()
}
pub fn term_show_multi_table(
  rows:Vec<Vec<String>>,
)->(){
  /*
  show tables independently;
  */
  term_show_rows(rows);
}
#[test]
pub fn try_show_table()->(){
  let title:Vec<String>=vec!["a".to_string(),"b".to_string(),"c".to_string()];
  let data:Vec<Vec<String>>=vec![
    vec![12.to_string(),4.to_string(),6.to_string()],
    vec![15.to_string(),8.to_string(),22.to_string()],
    vec![215.to_string(),88.to_string(),22.to_string()],
  ];
  term_show_table(title,data);
}
