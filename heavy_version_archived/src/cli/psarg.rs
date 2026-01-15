use std::ops::RangeInclusive;
use clap::builder::TypedValueParser;

use crate::meta::MatchMeta;
use crate::cli::NumRangeArg;

pub fn parse_shtmeta(
  single_meta:&str,
)->Result<MatchMeta,String>{
  todo!()
}
pub fn parse_num_range(
  arg_str:&str
)->Result<Vec<usize>,String>{
  /*
   * 2,4,11-13,9,21-13=>[2,4,11,12,13,9,21,22,23]
   */
  // Ok(Vec::from([0usize]))
  match arg_str{
    ""=>{Err("0".to_string())},
    _=>{Ok(
      arg_str.split(',')
        .flat_map(
          |part|{
            match part.split_once('-'){
              Some((start,end))=>{
                let start:usize=start.parse().unwrap();
                let end:usize=end.parse().unwrap();
                RangeInclusive::new(start,end).collect::<Vec<usize>>()
              },
              None=>{
                let part:usize=part.parse().unwrap();
                Vec::from([part])
              },
            }
          }
        )
        .collect()
    )},
  }
  
}
pub fn parse_num_range2(
  arg_str:&str
)->Result<Vec<usize>,String>{
  Ok(arg_str.split(',') // turns into iterator;
    .flat_map( // breaks into every parts and then starts mapping;
      |part|{
        if let Some((start,end))=part.split_once('-'){
          let start:usize=start.parse().map_err(|_|"invalid_number").expect("invalid left range");
          let end:usize=end.parse().map_err(|_|"invalid_number").expect("invalid right range");
          RangeInclusive::new(start,end).collect::<Vec<_>>()
        }else{
          // let num=part.parse().map_err(|_|"invalid_number").expect("Invalid number range!");
          // Ok(vec![num])
          vec![0usize]
        }
      }
    )
    // .flatten() // expand every level;
    .collect()
  )
}

impl TypedValueParser for NumRangeArg{
  type Value=Vec<usize>;
  fn parse_ref(
    self:&Self,
    cmd:&clap::Command,
    arg:Option<&clap::Arg>,
    value:&std::ffi::OsStr,
  )->Result<Self::Value,clap::error::Error>{
    todo!()
  }
}
#[test]
fn test_num_range_parse()->(){
  let a:&str="2,4,11-12,9,21-23";
  let b:&str="";
  assert_eq!(parse_num_range(a),Ok(Vec::from([2,4,11,12,9,21,22,23])));
  assert_eq!(parse_num_range(b),Err("0".to_string()));
}
