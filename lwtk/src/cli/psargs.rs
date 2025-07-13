use crate::brother::{ShtMeta,StrMchLine,NumCmpLine};
/*
#[derive(Debug,Clone,Copy)]
pub struct ShtMeta<'arg>{
  ifp:&'arg String,
  shtna:&'arg String,
}
#[derive(Debug,Clone,Copy)]
pub struct StrMchLine<'arg>{ // string-match line
  regex_str:&'arg String,
  match_cell:(usize,usize), // (row_index,col_index) starts from (1,1);
}
#[derive(Debug,Clone,Copy)]
pub struct NumCmpLine<'arg>{ // number-compare line
  cmp_exp:&'arg String, // expression to compare;
  cmp_cell:(usize,usize), // (row_index,col_index) starts from (1,1);
}
*/
fn parse_sht<'arg>(
    argstr:&'arg str
)->Result<ShtMeta<'arg>,String>{
    // input: --sht sheet1,path1
    // ouput:ShtMeta{ifp:path1,shtna:sheet1}
    let mut _shtna:String;
    let mut _ifp:String;
    let parts:Vec<&str>=argstr.split(',').map(|s|s.trim()).collect::<Vec<&str>>();
    if parts.len() != 2{
        return Err(format!(
            "Invalid argument: expected `sheet,path`, got `{}`",
            argstr
        ));
    }else{
        _shtna=parts[0].to_string();
        _ifp=parts[1].to_string();
        return Ok(
            ShtMeta{
                ifp:&'arg _ifp,
                shtna:&'arg _shtna,
            }
        );
    }
}
fn parse_str_condition<'arg>(
    argstr:&'arg str
)->Result<Vec<StrMchLine<'arg>>,String>{
    todo!()
}
fn parse_num_condition<'arg>(
    argstr:&'arg str
)->Result<Vec<NumCmpLine<'arg>>,String>{
    todo!()
}
