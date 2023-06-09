use regex::Regex;
use crate::reader::acct::Acct;
use crate::reader::chart::ChartData;
#[derive(Debug)]
pub struct Table{
}
impl Table{
    pub fn new()->Self{
        Self{}
    }
    pub fn get_amt(self:&Self,accid:String)->&ChartData{
        ChartData::blank()
    }
}
pub fn test()->(){
    let a1:Acct=Acct::new(
        "1002",
        "bank",
        true,
        1,
    );
    let b1:Table=Table::new();
    let c1:&ChartData=a1.get_amt(&b1);
    println!("{:?}",c1);
}
