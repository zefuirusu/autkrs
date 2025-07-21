pub mod xlshow;
pub mod xlmatch;
pub mod xlcmp;
pub mod xlwt;

use rayon::prelude::*;
use std::path::{Path};
use calamine::{open_workbook,Data,Xlsx,Xls,Range,Reader,XlsxError,XlsError};

#[derive(Debug)]
pub enum ArgErr{
  IfpErr,
  ShtErr,
}
#[derive(Debug,Clone,Copy)]
pub struct ShtMeta<'arg>{
  shtna:&'arg String,
  ifp:&'arg String,
}
impl<'arg> ShtMeta<'arg>{
  pub fn new(_shtna:&'arg String,_ifp:&'arg String,)->Self{
    Self{shtna:_shtna,ifp:_ifp,}
  }
}
#[derive(Debug,Clone,Copy)]
pub struct StrMchLine<'arg>{ // string-match line
  regex_str:&'arg String,
  match_cell:(usize,usize), // (row_index,col_index) starts from (1,1);
}
impl<'arg> StrMchLine<'arg>{
  pub fn new(_regex_str:&'arg String,_match_cell:(usize,usize))->Self{
    Self{regex_str:_regex_str,match_cell:_match_cell}
  }
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
)->Result<Range<Data>,String>{
  let path:&'xl Path=Path::new(sht.ifp.as_str());
  match extsn(&sht.ifp){
    Some(_any_extension)=>{
      match _any_extension{
        "xlsx"=>{
          // type rdtp=Xlsx<calamine::Reader<std::io::BufReader<std::fs::File>>>;
          // open_workbook::<rdtp,&Path>(path)
          match open_workbook::<Xlsx<_>,&Path>(path){
            Ok(mut _reader)=>{
              match _reader.worksheet_range(sht.shtna.as_str()){
                Ok(_range)=>{return Ok(_range)},
                _=>{return Err(format!("Invalid sheet: {}",sht.shtna))}
              }
            },
            _=>{return Err(format!("Invalid path: {}",sht.ifp))},
          }
        },
        "xls"=>{
          match open_workbook::<Xls<_>,&Path>(path){
            Ok(mut _reader)=>{
              match _reader.worksheet_range(sht.shtna.as_str()){
                Ok(_range)=>{return Ok(_range)},
                _=>{return Err(format!("Invalid sheet: {}",sht.shtna))}
              }
            },
            _=>{return Err(format!("Invalid path: {}",sht.ifp))},
          }
        },
        _=>{return Err(format!("Invalid file type: {}",_any_extension.to_string()))}
      }
    },
    None=>{return Err(format!("Invalid path, no extension."))},
  };
}
