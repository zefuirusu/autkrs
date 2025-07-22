// use crate::brother::{ShtMeta,StrMchLine,NumCmpLine};
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
    let parts:Vec<&str>=argstr.split(',')
        .map(|s|s.trim())
        .collect::<Vec<&str>>();
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
)->Result<(String,(usize,usize)),String>{//(regex_str,(rdx,cdx))
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
)->Result<(String,(usize,usize)),String>{// (cmp_exp,(rdx,cdx))
    todo!()
}
pub fn parse_range<'arg>(
    argstr:&'arg str,
)->Result<Vec<usize>,String>{
    /*
    Currently, this function is not being used at `/src/cli/cmd.rs`;
    */
    let mut num:Vec<usize>=Vec::new();
    for part in argstr.split(','){
        if part.contains('-'){
            let range: Vec<&str> = part.split('-').collect::<Vec<&str>>();
            if range.len()  != 2 {
                return Err(format!("Invalid range format: {}", part));
            }else{
                match range[0].parse::<usize>(){
                    Ok(_left)=>{
                        match range[1].parse::<usize>(){
                            Ok(_right)=>{
                                num.extend((_left..=_right).collect::<Vec<usize>>());
                            },
                            _=>{
                                // num.push(0usize);
                                return Err(format!("Invalid ceiling value: {}", range[1]));
                            },
                        }
                    },
                    _=>{
                        // num.push(0usize);
                        return Err(format!("Invalid floor value: {}", range[0]));
                    }
                }
            }
        }else{
            match part.parse::<usize>(){
                Ok(_v)=>{
                    num.push(_v);
                },
                _=>{
                    // num.push(0usize);
                    return Err(format!("Invalid arg:{}",part));
                },
            }
        }
    }
    println!("check passed num:");
    crate::utils::check(&num);
    match num.len(){
        0=>{
            return Err(format!("No args parsed:{:?}",&num));
        },
        _=>{
            // let num:Vec<usize>=num.into_iter().filter(|x|{*x !=0usize}).collect::<Vec<usize>>();
            return Ok(num);
        }
    }
}
#[test]
fn test_parse_range()->(){
    assert_eq!(
        parse_range(
            "3,4,9-11,6,23-25,5"
        ).unwrap(),
        vec![3,4,9,10,11,6,23,24,25,5],
    );
}
