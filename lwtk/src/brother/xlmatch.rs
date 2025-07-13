use std::path::{Path};
use calamine::{Range,Data};
use rayon::prelude::*;
use regex::Regex;

use crate::brother::{StrMchLine,ShtMeta};
use crate::brother::{value2str,vecb_judge,get_sht_data};

// single row,single condition line:
fn single_row_match<'mch>(
  _row:&'mch [Data],
  _line:&'mch StrMchLine,
)->bool{
  let match_cell=_row.get(_line.match_cell.1 as usize -1)
    .expect("Match column index out of range.");
  let text_match_col=value2str(&match_cell);
  let _regex=Regex::new(_line.regex_str.as_str()).expect("Fail to compile regex!");
  _regex.is_match(&text_match_col)
}
// single row, multiple condition(lines):
fn multi_rows_match<'mch>(
  _row:&'mch [Data],
  _lines:&'mch Vec<StrMchLine>,
)->bool{
  vecb_judge(
    &_lines.iter().map(
      |line|{single_row_match(_row,line)}
    ).collect::<Vec<bool>>() // result of collect:Vec<bool>;
  )
}
// single range(multiple rows), single condition line:
fn base_filter<'f>( // f:filter
  _rg:&'f Range<Data>,
  _line:&'f StrMchLine,
)->Vec<Vec<String>>{
  _rg.rows()
    .par_bridge()
    .filter_map(
      |row|{
        match single_row_match(&row,_line){
          true=>Some(
            row.into_iter()
            .map(
              |cell|{
                value2str(cell)
                  // cell
              }
            ).collect::<Vec<_>>()
          ),
          false=>Option::None,
        }
      }
    ).collect()
}
// single range(multiple rows), multiple condition line:
fn adv_filter<'f>( // f:filter
  _rg:Range<Data>,
  _lines:&'f Vec<StrMchLine>,
)->Vec<Vec<String>>{
  _rg.rows()
    .par_bridge()
    .filter_map(
      |row|{
        match multi_rows_match(&row,_lines){
          true=>{
            Some(
              row.into_iter().map(
                |cell|{
                  value2str(cell)
                }
              ).collect::<Vec<_>>()
            )
          },
          false=>{
            Option::None
          },
        }
      }
    ).collect()
}
// multiple sheets(ranges), multiple conditions:
fn multi_sht_match<'f>(
  _lines:&'f Vec<StrMchLine>,
  _shtli:&'f Vec<ShtMeta>,
)->Vec<Vec<String>>{
  _shtli.par_iter().map(
    |sht|{
        adv_filter(
            get_sht_data(sht),
            _lines,
        )
    }
  ).flatten().collect()
}

// This function seems useless:
// single sheet(range,multiple rows), single condition:
/*
pub fn sht_str_match<'sm>( // sm:str-match
  line:&'sm StrMchLine,
  sht:&'sm ShtMeta,
)->Vec<Vec<String>>{
  base_filter(
    &get_sht_data(sht,),
    line,
  )
}
*/
// single sheet(range), multi-conditions:
pub fn sht_num_compare<'nc>(
)->(){ // nc:number-compare
  todo!()
}

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
// #[test]
pub fn test_match()->(){
  let pstr:&str="/home/debvbx/Documents/cd_gl_2024_1-11.xlsx";
  // let p1:&Path=Path::new(pstr);
  // let p2:PathBuf=PathBuf::from(pstr);
  // crate::utils::check(&p1.extension().unwrap());
  // crate::utils::check(&p2.capacity());
  // let wb:Result<Xlsx<_>,calamine::XlsxError>=calamine::open_workbook(p1);
  // crate::utils::check(&wb);
  let resu:Vec<Vec<String>>=sht_str_match(
    &StrMchLine{regex_str:&String::from("现金"),match_cell:(4usize,5usize)},
    &ShtMeta{ifp:&pstr.to_string(),shtna:&String::from("Sheet1")},
  );
  println!("{:?}",resu);
}
pub fn test_multi_match()->(){
  let pstr="/home/debvbx/Documents/cd_gl_2024_1-11.xlsx".to_string();
  let s1="^现金.*".to_string();
  let s2="年初余额".to_string();
  let s3="Sheet1".to_string();
  let _lines:Vec<StrMchLine>=vec![
    StrMchLine{regex_str:&s1,match_cell:(1usize,5usize)},
    StrMchLine{regex_str:&s2,match_cell:(1usize,10usize)},
  ];
  let _shtli:Vec<ShtMeta>=vec![
    ShtMeta{ifp:&pstr,shtna:&s3},
  ];
  let resu:Vec<Vec<String>>=multi_sht_match(
    &_lines,
    &_shtli,
  );
  println!("{:?}",&resu.len());
  for row in &resu{
    println!("{:?}",&row);
  }
}
pub fn test2_multi_match()->(){
    // 找到问题所在了，设计是求交集，运行结果是求并集；
    let pstr="/home/debvbx/Documents/purchase_report.xlsx".to_string();
    let shtna="data".to_string();
    let s1="^蜜蜂牌".to_string();
    let s2="爱坤新凯".to_string();
    let _lines:Vec<StrMchLine>=vec![
        StrMchLine{regex_str:&s1,match_cell:(1usize,3usize)},
        StrMchLine{regex_str:&s2,match_cell:(1usize,6usize)},
    ];
    let _shtli:Vec<ShtMeta>=vec![
        ShtMeta{ifp:&pstr,shtna:&shtna},
    ];
  let resu:Vec<Vec<String>>=multi_sht_match(
    &_lines,
    &_shtli,
  );
  println!("{:?}",&resu.len());
  for row in &resu{
    println!("{:?}",&row);
  }
}
