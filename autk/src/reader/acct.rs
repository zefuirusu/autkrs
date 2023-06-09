use String;
#[derive(Debug)]
pub struct Acct{
    pub accid:String, 
    pub accna:String,
    is_cr:bool,
    acclv:u8,
}
impl Acct{
    pub fn new(accid:&str,accna:&str,level:u8)->Acct{
        Acct { accid: String::from(accid), accna: String::from(accna), acclv: level }
    }
}