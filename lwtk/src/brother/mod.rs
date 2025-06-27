pub mod xlmatch;
pub mod xlshow;

use calamine::Data;

#[derive(Debug)]
pub struct ShtMeta{
  ifp:String,
  shtna:String,
}
#[derive(Debug)]
pub struct StrMchLine{ // string-match line
  regex_str:String,
  match_col:usize,
}
#[derive(Debug)]
pub struct NumCmpLine{ // number-compare line
  // num_col:usize,
}

pub fn value2str(
  cell_value:&Data,
)->String{
  match cell_value{
      Data::Int(i) => i.to_string(),
      Data::Float(f) => f.to_string(),
      Data::String(s) => s.clone(),
      Data::Bool(b) => if *b { "true".to_string() } else { "false".to_string() },
      Data::DateTime(_exceldatetime) => _exceldatetime.to_string(),
      Data::DateTimeIso(dt) => format!("{}", dt),
      Data::DurationIso(d) => format!("{}", d),
      // Data::Empty => "".to_string(),
      _ => "".to_string(),
  }
}
