use crate::calculation::{ChartData,Table};
impl Table{
    pub fn new()->Self{
        Self{}
    }
    pub fn get_amt(self:&Self,accid:String)->&ChartData{
        ChartData::blank()
    }
}
