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
)->Result<(String,String),String>
    /*
    input: --sht path1,sheet1
    ouput:(path1,sheet1)
    */
{
    let parts:Vec<&str>=argstr.split(',')
        .map(|s|s.trim())
        .collect::<Vec<&str>>();
    if parts.len() != 2{
        return Err(
            format!(
                "Invalid argument: expected `path,sheet`, got `{}`",
                argstr
            )
        );
    }else{
        let   _ifp=parts[0].to_string();
        let _shtna=parts[1].to_string();
        return Ok(
            (_ifp,_shtna)
        );
    }
}
pub fn parse_str_condition<'arg>(
    argstr:&'arg str
)->Result<(usize,usize,String),String>
    /*
    input: --line row_index,col_index,regstr
    output: (row_index,col_index,regstr)
    */
{
    let parts:Vec<&str>=argstr.split(',').map(
        |s|s.trim()
    ).collect::<Vec<&str>>();
    if parts.len() !=3{
        return Err(
            format!(
                "Invalid argument: expected `row_index,col_index,regex_str`, got `{}`",
                argstr,
            )
        )
    }else{
        let _row_index:usize=parts[0].parse::<usize>().expect("Invalid row index");
        let _col_index:usize=parts[1].parse::<usize>().expect("Invalid column index");
        let _regstr:String=parts[2].to_string();
        return Ok(
            (_row_index,_col_index,_regstr)
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
