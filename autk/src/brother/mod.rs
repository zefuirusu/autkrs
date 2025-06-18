pub mod bk;
pub mod xlmatch;
pub mod xlshow;
pub struct BkBro{}

use calamine::DataType;

pub fn cell_value2string(
  cell_value:&DataType,
)->String{
  match cell_value{
      DataType::String(s) => s.clone(),
      DataType::Float(f) => f.to_string(),
      DataType::Int(i) => i.to_string(),
      DataType::Bool(b) => if *b { "true".to_string() } else { "false".to_string() },
      DataType::DateTime(f) => f.to_string(),
      DataType::Empty => "".to_string(),
      DataType::Duration(d) => format!("{:?}", d),
      DataType::DateTimeIso(dt) => format!("{}", dt),
      DataType::DurationIso(d) => format!("{}", d),
      _ => "".to_string(),
  }
}
