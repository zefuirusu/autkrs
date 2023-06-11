use std::collections::BTreeMap;
#[derive(Debug)]
pub struct ChartData{
    st:f64,
    dr:f64,
    cr:f64,
    end:f64,
}
impl ChartData{
    pub fn blank<'a>()->&'a Self{
        &Self{
            st:0.0,
            dr:0.0,
            cr:0.0,
            end:0.0,
        }
    }
    fn cr_balance(self:&Self)->bool{
        self.st-self.dr+self.cr-self.end<0.004
    }
    fn de_blance(self:&Self)->bool{
        self.st+self.dr-self.cr-self.end<0.004
    }
}
#[derive(Debug)]
pub struct Chart{
    name:String,
    cols:Vec<String>,
    data:BTreeMap<String,ChartData>,
}
pub struct GlData{}
pub struct GL{}
