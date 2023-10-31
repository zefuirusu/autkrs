use crate::calculation::ChartData;
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
