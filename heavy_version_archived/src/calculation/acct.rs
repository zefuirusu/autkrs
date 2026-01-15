use crate::calculation::{Acct,AcctAmt,ChartData,Table};
impl Acct{
    pub fn new(accid:&str,accna:&str,is_cr:bool,level:u8)->Self{
        Self{
            accid: String::from(accid),
            accna: String::from(accna),
            is_cr: is_cr,
            acclv: level,
        }
    }
    pub fn get_amt<'a>(
        self:&Self,
        from_table:&'a Table
    )->&'a ChartData{
        from_table.get_amt(self.accid.clone())
    }
}
impl AcctAmt{
    pub fn blank()->Self{
        Self { start: 0.0, debit: 0.0, credit: 0.0, end: 0.0 }
    }
    pub fn is_bal(self:&Self)->bool{
        self.start+self.debit-self.credit-self.end<0.004
    }
}
