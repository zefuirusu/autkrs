use crate::brother::{ShtMeta,StrMchLine,NumCmpLine};
/*
TODO
#[derive(Debug,Clone,Copy)]
pub struct ShtMeta<'arg>{
  ifp:&'arg String,
  shtna:&'arg String,
}
TODO
#[derive(Debug,Clone,Copy)]
pub struct StrMchLine<'arg>{ // string-match line
  regex_str:&'arg String,
  match_cell:(usize,usize), // (row_index,col_index) starts from (1,1);
}
TODO
#[derive(Debug,Clone,Copy)]
pub struct NumCmpLine<'arg>{ // number-compare line
  cmp_exp:&'arg String, // expression to compare;
  cmp_cell:(usize,usize), // (row_index,col_index) starts from (1,1);
}
*/
pub fn parse_sht<'arg>(
    argstr:&'arg str
)->Result<(String,String),String>{
    /*
    input: --sht sheet1,path1
    ouput:(sheet1,path1)
    */
    let parts:Vec<&str>=argstr.split(',').map(|s|s.trim()).collect::<Vec<&str>>();
    if parts.len() != 2{
        return Err(
            format!(
                "Invalid argument: expected `sheet,path`, got `{}`",
                argstr
            )
        );
    }else{
        return Ok(
            (parts[0].to_string(),parts[1].to_string())
        );
    }
}
pub fn parse_str_condition<'arg>(
    argstr:&'arg str
)->Result<(String,(usize,usize)),String>{
    /*
    input: --line regstr,row_index,col_index
    output: (regstr,(row_index,col_index))
    */
    let parts:Vec<&str>=argstr.split(',').map(
        |s|s.trim()
    ).collect::<Vec<&str>>();
    if parts.len() !=3{
        return Err(
            format!(
                "Invalid argument: expected `regex_str,row_index,col_index`, got `{}`",
                argstr,
            )
        )
    }else{
       return Ok(
           (
               parts[0].to_string(),
               (
                   parts[1].parse::<usize>().expect("invalid row index"),
                   parts[2].parse::<usize>().expect("invalid column index"),
               )
           )
       ) 
    }
}
pub fn parse_num_condition<'arg>(
    argstr:&'arg str,
)->Result<Vec<NumCmpLine<'arg>>,String>{
    todo!()
}
pub fn parse_range<'arg>(
    argstr:&'arg str,
)->Result<Vec<usize>,String>{
    let mut num:Vec<usize>=Vec::new();
    for part in argstr.split(','){
        if part.contains('-'){
            let range: Vec<&str> = part.split('-').collect();
            if range.len()  != 2 {
                return Err(format!("Invalid range: {}", part));
            }
            match range[0].parse::<usize>(){
                Ok(rdx)=>{
                    match range[1].parse::<usize>(){
                        Ok(cdx)=>{
                            num.extend((rdx..=cdx).collect::<Vec<usize>>());
                        },
                        _=>{return Err(format!("Invalid range: {}", range[0]));},
                    }
                },
                _=>{return Err(format!("Invalid range: {}", range[0]));}
            }
        }else{
            num.push(
                part.parse::<usize>().unwrap()
            );
        }
    }
    match num.len(){
        0=>{Err("Invalid range!".to_string())},
        _=>{Ok(num)}
    }
}
