use std::any::type_name;
use std::mem::{size_of,align_of_val};
#[derive(Debug,Clone)]
struct LayoutInfo<T>
    where T:std::fmt::Debug,
{
    value:T,
    typename:String,
    size:usize,
    align:usize,
    division:usize,
}
impl<'a,T> LayoutInfo<&'a T>
    where T:std::fmt::Debug,
{
    fn new(t:&'a T)->Self{
        Self{
            value:t,
            typename:type_name::<T>().to_string(),
            size:size_of::<T>(),
            align:align_of_val(t),
            division:size_of::<T>()/align_of_val(t),
        }
    }
}
fn check<'a,T>(t:&'a T)->()
where T:std::fmt::Debug,
{
    let l=LayoutInfo::new(t);
    println!("{:?}",l);
}
#[derive(Debug,Clone)]
struct San{
    name:String,
    des:String,
    value:i32,
}
impl San{
    fn new()->Self{
        San{
            name:"wrong".to_string(),
            des:"SanFrom;".to_string(),
            value:1024i32,
        }
    }
}
pub fn test() {
    let a=San::new();
    check(&a);
}
