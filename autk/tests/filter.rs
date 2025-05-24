use std::path::Path;
use calamine::{open_workbook,Xlsx,Reader,DataType};
fn filter_by_text(
  item:&str,
  col_num:usize,
  shtna:&str,
  ifp:&str,
)->Vec<Vec<DataType>>{
  let mut wb:Xlsx<_> = open_workbook(ifp).expect("check the input file path.");
  let range = wb.worksheet_range(shtna).expect("failed to get range.");
  range.expect("failed to get range.").rows().filter(
    |row|{
      row.get(col_num)
        .and_then(|v| v.as_string())
        .map(|s| s.contains(item))
        .unwrap_or(false)
    }
  ).map(|r| r.to_vec()).collect()
}
#[test]
fn try_load()->(){
  let p=Path::new("/home/debvbx/Documents/autkrs/autk/tests/yl_rm_2021.xlsx");
  let resu=filter_by_text(
    "YLD",
    2,
    "inventory2020",
    &p.to_str().expect(""),
  );
  println!("search at workbook:{:?}, find:{:?}",&p.to_str(),&resu);
}
