use std::collections::BTreeMap;

#[derive(Debug,Clone)]
pub struct ShtMeta<'meta>{
  shtna:&'meta str,
  title:usize,
  ifp:&'meta str,
}
#[derive(Debug,Clone)]
pub struct MatchMeta<'meta>{
  match_col:usize,
  shtli:Vec<&'meta ShtMeta<'meta>>,
}
/*
pub trait ArgParse{
  fn parse(
    arg_input:&str
  )->Result<MatchMeta,&str>{
    todo!()
  }
}
impl ArgParse for MatchMeta{}
*/

pub trait Meta{
    fn keep_additional()->bool{false}
    // fn load()->BTreeMap<String,u8>;
}
pub struct XlMeta{
    path:&'static str,
    data:BTreeMap<String,u8>,
    pure_file_name:&'static str,
    suffix:&'static str,
}
pub struct DirMeta{// all Excel files in the directory;
    path:&'static str,
    data:BTreeMap<String,u8>,
    common_title:u8,
}
pub struct JsonMeta{
    // a json file indicating Excel file paths,sheet names,title locations,etc.
    // {
        // "file1":[["sheet1",0],["sheet1",4]],
        // "file1":[["sheet3",10]],
    // }
    path:&'static str,
    data:BTreeMap<String,u8>,
}


impl Meta for XlMeta{
    fn keep_additional()->bool{false}
}
impl Meta for DirMeta{
    fn keep_additional()->bool{true}
}
impl Meta for JsonMeta{
    fn keep_additional()->bool{true}
}
