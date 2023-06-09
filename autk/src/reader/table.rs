use std::collections::BTreeMap;
#[derive(Debug)]
pub struct ChartData{
    st:f64,
    de:f64,
    cr:f64,
    end:f64,
}
#[derive(Debug)]
pub struct Chart{
    name:String,
    cols:Vec<String>,
    data:BTreeMap<String,ChartData>,
}
impl ChartData{
    fn cr_balance(self:&Self)->bool{
        self.st-self.de+self.cr-self.end<0.004
    }
    fn de_blance(self:&Self)->bool{
        self.st+self.de-self.cr-self.end<0.004
    }
}
pub struct GlData{}
pub struct GL{}