pub mod xlshow;
pub mod xlmatch;
pub mod xlcmp;

use rayon::prelude::*;
use std::path::{Path};
use calamine::{open_workbook,Data,Xlsx,Xls,Range,Reader};

#[derive(Debug,Clone,Copy)]
pub struct ShtMeta<'arg>{
  ifp:&'arg String,
  shtna:&'arg String,
}
#[derive(Debug,Clone,Copy)]
pub struct StrMchLine<'arg>{ // string-match line
  regex_str:&'arg String,
  match_cell:(usize,usize), // (row_index,col_index) starts from (1,1);
}
#[derive(Debug,Clone,Copy)]
pub struct NumCmpLine<'arg>{ // number-compare line
  cmp_exp:&'arg String, // expression to compare;
  cmp_cell:(usize,usize), // (row_index,col_index) starts from (1,1);
}
pub fn extsn<'p>(
  ifp:&'p str,
)->Option<&'p str>{
  match Path::new(ifp).extension(){
    Some(_has_extension)=>{
      match _has_extension.to_str(){ // &OsStr.to_str(&self)->Option<&str>
        Some(_any_extension)=>{
          Some(_any_extension)
        },
        None=>{None}
      }
    },
    None=>{None}
  }
}
// judge Vec<bool>:
pub fn vecb_judge<'mch>( // judge Vec<bool>;
  v:&'mch Vec<bool>
)->bool{
  v.par_iter().all(|&b|b)
}
pub fn value2str(
  cell_value:&Data,
)->String{
  match cell_value{
      Data::Int(i) => i.to_string(),
      Data::Float(f) => f.to_string(),
      Data::String(s) => s.clone(),
      Data::Bool(b) => if *b { "true".to_string() } else { "false".to_string() },
      Data::DateTime(_exceldatetime) => _exceldatetime.to_string(),
      Data::DateTimeIso(dt) => format!("{}", dt),
      Data::DurationIso(d) => format!("{}", d),
      // Data::Empty => "".to_string(),
      _ => "".to_string(),
  }
}
// get Range<Data> from ShtMeta:
pub fn get_sht_data<'xl>( // sheet to range
  sht:&'xl ShtMeta,
)->Range<Data>{
  let path:&'xl Path=Path::new(sht.ifp.as_str());
  let range=match extsn(&sht.ifp){
          Some(_any_extension)=>{
            match _any_extension{
              "xlsx"=>{
                // type rdtp=Xlsx<calamine::Reader<std::io::BufReader<std::fs::File>>>;
                // open_workbook::<rdtp,&Path>(path)
                open_workbook::<Xlsx<_>,&Path>(path)
                .expect("check the input path!")
                .worksheet_range(sht.shtna.as_str())
                .expect("Sheet may not exist!")
              },
              "xls"=>{
                open_workbook::<Xls<_>,&Path>(path)
                .expect("check the input path!")
                .worksheet_range(sht.shtna.as_str())
                .expect("Sheet may not exist!")
              },
              _=>{calamine::Range::default()}
            }
          },
          None=>{calamine::Range::default()},
  };
  return range;
}
