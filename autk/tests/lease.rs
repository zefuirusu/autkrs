use std::collections::BTreeMap;
use std::cmp::{Ordering,PartialEq,PartialOrd};

type AmtSeries=BTreeMap<Sdf,f64>;

#[derive(Debug,Clone,Eq,PartialEq,PartialOrd)]
struct Sdf(u32,u8,u8);// simple date format;
impl Sdf{
    fn new_blank()->Self{
        Sdf(1900u32,1u8,1u8)
    }
    fn from3u(y:u32,m:u8,d:u8)->Self{
        Sdf(y,m,d)
    }
    fn month_diff(
        self:&Self,
        other:&Self,
    )->i32{
        (12*(self.0-other.0) as i32)+
        (self.1 as i32)-(other.1 as i32)+{
            if (self.2 as i32) -(other.2 as i32) > 14
            {1i32}else{0i32}
        }
          
    }
}
impl std::cmp::Ord for Sdf{
    fn cmp(self:&Self,other:&Self)->Ordering{
        match self.0.cmp(&other.0){
            Ordering::Less=>Ordering::Less,
            Ordering::Greater=>Ordering::Greater,
            Ordering::Equal=>{
                match self.1.cmp(&other.1){
                    Ordering::Less=>Ordering::Less,
                    Ordering::Greater=>Ordering::Greater,
                    Ordering::Equal=>{
                        match self.2.cmp(&other.2){
                            Ordering::Less=>Ordering::Less,
                            Ordering::Greater=>Ordering::Greater,
                            Ordering::Equal=>Ordering::Equal,
                        }
                    },
                }
            },
        }
    }
}
#[derive(Debug)]
struct LeaseContract{
    lessor:&'static str,
    lessee:&'static str,
    start_date:Sdf,
    expired_date:Sdf,
    plan:AmtSeries,
}
impl LeaseContract{
    fn new(
        _lessor:&'static str,
        _lessee:&'static str,
        _start_dt:(u32,u8,u8),
        _plan:AmtSeries,
    )->Self{
        LeaseContract{
            lessor:_lessor,
            lessee:_lessee,
            start_date:Sdf::from3u(_start_dt.0,_start_dt.1,_start_dt.2),
            expired_date:match _plan.iter().last(){
                Option::None=>{Sdf::new_blank()},
                Option::Some(_last_pay)=>{_last_pay.0.clone()}
            },
            plan:_plan,
        }
    }
}
fn roua_init_amt<'contract>(
    rate:f64,
    lc:&'contract LeaseContract,
)->f64{
    lc.plan.iter().map(
        |(date,payment)|
        (1f64+rate).powf(
            -(date.month_diff(&lc.start_date) as f64).max(0f64)
        )*payment
    ).sum::<f64>()
}
fn roua_bal(
    roua_start_bal:f64,
    expired_date:Sdf,
)->AmtSeries{
    todo!()
}
fn ufe_init()->f64{
    todo!()
}
fn ufe_bal()->AmtSeries{
    todo!()
}
fn main()->(){
    println!("Hello! Lease Liability Calculator.");
    //payment plan
    let pp=AmtSeries::from([
        (Sdf::from3u(2023,1,5),12000f64),
        (Sdf::from3u(2023,12,15),12000f64),
        (Sdf::from3u(2024,4,15),12000f64),
        (Sdf::from3u(2024,7,15),12000f64),
    ]);
    //lease contract
    let lc=LeaseContract::new(
        "A",
        "YiZhun",
        (2023,10,1),
        pp,
    );
    println!("{:?}",&lc);
    assert_eq!(roua_init_amt(0.049f64,&lc),39712.96677034796f64);
}