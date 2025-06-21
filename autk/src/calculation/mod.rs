pub mod acct;
pub mod chart;
pub mod table;
pub mod xlsht;
pub mod calsht;
pub mod chsht;

use std::collections::BTreeMap;

#[derive(Debug)]
struct RegCondiLine{ // condition line for regular expression;
  regex_str:Box<String>,
  match_col:usize,
}
#[derive(Debug)]
pub struct RegCondiMatrix<'line>{ // condition matrix for regular expression;
  line:&'line RegCondiLine,
}

#[derive(Debug)]
pub struct XlSheet<'map>{
    xlmap:&'map BTreeMap<String,u8>,
    // data:Option<DataFrame>,
    nick_name:&'map str,
}
#[derive(Debug)]
pub struct CalSheet<'map>{
    xlmap:&'map BTreeMap<String,u8>,
    nick_name:&'map str,
}
#[derive(Debug)]
pub struct ChSheet<'map>{
    xlmap:&'map BTreeMap<String,u8>,
    nick_name:&'map str,
}

#[derive(Debug)]
pub struct Acct{
    accid:String, 
    accna:String,
    is_cr:bool,
    acclv:u8,
}
#[derive(Debug)]
pub struct AcctAmt{
    start:f64,
    debit:f64,
    credit:f64,
    end:f64,
}
#[derive(Debug)]
pub struct ChartData{
    st:f64,
    dr:f64,
    cr:f64,
    end:f64,
}
#[derive(Debug)]
pub struct Chart{
    name:String,
    cols:Vec<String>,
    data:BTreeMap<String,ChartData>,
}
#[derive(Debug)]
pub struct Table{
}
#[derive(Debug)]
pub struct GlData{}
#[derive(Debug)]
pub struct GL{}
