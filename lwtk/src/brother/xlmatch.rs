use std::path::{Path,PathBuf};
use calamine::{open_workbook,Reader,Xlsx,Xls,Range,Data};
use rayon::prelude::*;
use regex::Regex;

use crate::brother::value2str;
use crate::brother::{StrMchLine,ShtMeta};

// type WocRow=Vec<Vec<Data>>;
// impl rayon::iter::FromParallelIterator<Data> for WocRow{}

fn judge<'mch>(v:&'mch Vec<bool>)->bool{
  v.par_iter().all(|&b|b)
}
fn single_match<'mch>(
  _row:&'mch [Data],
  _line:&'mch StrMchLine,
)->bool{
  let match_cell=_row.get(_line.match_col as usize -1)
    .expect("Match column index out of range.");
  let text_match_col=value2str(&match_cell);
  let _regex=Regex::new(_line.regex_str.as_str()).expect("Fail to compile regex!");
  _regex.is_match(&text_match_col)
}
fn range_filter<'rf>( // rf:range filter
  _rg:&'rf Range<Data>,
  _line:&'rf StrMchLine,
)->Vec<Vec<String>>{
  _rg.rows()
    .par_bridge()
    .filter_map(
      |row|{
        match single_match(&row,_line){
          true=>Some(
            row.into_iter().map(|cell|{value2str(cell)}).collect::<Vec<_>>()
          ),
          false=>Option::None,
        }
      }
    ).collect()
}
fn adv_range_filter<'rf>( // rf:range filter
  _rg:calamine::Range<Data>,
  _lines:&'rf Vec<StrMchLine>
)->Vec<Vec<String>>{
  _rg.rows()
    .par_bridge()
    .filter_map(
      |row|{
        let multi_match_resu:Vec<bool>=_lines.par_iter().map(
          |line|{
            let match_cell=row.get(line.match_col as usize -1)
              .expect("Match column index out of range!");
            let text_match_col=value2str(&match_cell);
            let regex_item=Regex::new(line.regex_str.as_str()).expect("Fail to compile regex!");
            regex_item.is_match(&text_match_col)
          }
        ).collect::<Vec<bool>>();
        match judge(&multi_match_resu){
          true=>{
            Some(
              row.into_iter().map(|cell|{value2str(cell)}).collect::<Vec<_>>()
            )
          },
          false=>{
            Option::None
          },
        }
      }
    ).collect()
}
fn load_xl<'xl>(
  sht:&'xl ShtMeta,
)->&Range<Data>{
  //TODO move key parts of col_str_match to here;
  todo!()
}
pub fn col_str_match<'sm>( // sm:str-match
  line:&'sm StrMchLine,
  sht:&'sm ShtMeta,
)->Vec<Vec<String>>{
  let range:Range<Data>;
  let path:&Path=Path::new(sht.ifp.as_str());
  if let Some(_has_extension)=path.extension(){
    if let Some(_any_extension)=_has_extension.to_str(){
      match _any_extension{
        "xlsx"=>{
          let wb:Result<calamine::Xlsx<_>,calamine::XlsxError>=open_workbook(path);
          range=wb
            .expect("Impossible! Check the input path.")
            .worksheet_range(sht.shtna.as_str())
            .expect("Get error when calling worksheet_range() for Xlsx; Generally it cannot happen.");
          return range_filter(&range,line);
        },
        "xls"=>{
          let wb:Result<calamine::Xls<_>,calamine::XlsError>=open_workbook(path);
          range=wb
            .expect("Impossible! Check the input path.")
            .worksheet_range(sht.shtna.as_str())
            .expect("Get error when calling worksheet_range() for Xls; Generally it cannot happen.");
          return range_filter(&range,line);
        },
        _=>{return vec![vec![]];}
      }
    }else{return vec![vec![]];}
  }else{return vec![vec![]];}
}
pub fn col_num_compare<'nc>(
)->(){ // nc:number-compare
  todo!()
}

/// match specific column in Excel by regex;
// pub fn match_col(
  // regex_str:String,
  // match_col:usize,
  // shtna:String,
  // ifp:String,
// )->Vec<Vec<String>>{
  // let regex_item=Regex::new(&regex_str).unwrap();
  // let mut wb:Xlsx<_>=open_workbook(PathBuf::from(ifp.as_str()))
    // .expect("Cannot open the target Excel!");
  // let range:calamine::Range<_>=wb.worksheet_range(&shtna)
    // .expect("Cannot get range!");
  // range
    // .rows()
    // .par_bridge()
    // .filter_map(
      // |row|{
        // let match_cell=row.get(match_col as usize -1)
          // .expect("Match column index out of range.");
        // let text_match_col=value2str(&match_cell);
        // if regex_item.is_match(&text_match_col){
          // Some(
            // row.into_iter().map(|cell|{value2str(cell)}).collect::<Vec<_>>()
          // )
        // }else{
          // Option::None
        // }
      // }
    // ).collect()
// }
pub fn multi_col_str_match<'sm>( // sm:str-match
  lines:&'sm Vec<StrMchLine>,
  sht:&'sm ShtMeta
)->Vec<Vec<String>>{
  let range:Range<Data>;
  let path:&Path=Path::new(sht.ifp.as_str());
  //TODO simulate col_str_match;
  let mut resu:Vec<Vec<String>>=Vec::new();
  return resu;
}
pub fn multi_col_num_compare()->(){
  todo!()
}
// #[test]
pub fn test_match()->(){
  let pstr:&str="/home/debvbx/Documents/cd_gl_2024_1-11.xlsx";
  // let p1:&Path=Path::new(pstr);
  // let p2:PathBuf=PathBuf::from(pstr);
  // crate::utils::check(&p1.extension().unwrap());
  // crate::utils::check(&p2.capacity());
  // let wb:Result<Xlsx<_>,calamine::XlsxError>=calamine::open_workbook(p1);
  // crate::utils::check(&wb);
  let resu:Vec<Vec<String>>=col_str_match(
    &StrMchLine{regex_str:String::from("现金"),match_col:5usize},
    &ShtMeta{ifp:pstr.to_string(),shtna:String::from("Sheet1")},
  );
  println!("{:?}",resu);
}
pub fn test_multi_match()->(){
  let pstr:&str="/home/debvbx/Documents/cd_gl_2024_1-11.xlsx";
  let _lines:Vec<StrMchLine>=vec![
    StrMchLine{regex_str:String::from("现金"),match_col:5usize},
    StrMchLine{regex_str:String::from("年初余额"),match_col:10usize},
  ];
  let _shtli:Vec<ShtMeta>=vec![
    ShtMeta{ifp:pstr.to_string(),shtna:String::from("Sheet1")},
  ];
}
