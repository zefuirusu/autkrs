use clap::{Parser,Subcommand,Args};

use crate::brother::{ShtMeta,StrMchLine};

#[derive(Debug,Parser)]
#[
  command(
    author="Skandha Zhu",
    name="autk",
    version="1.0.1",
    about="AUTK, rust version.",
    long_about="Auditors' Toolkit.",
  )
]struct BaseCmd{
  #[command(subcommand)]
  cmd:BaseSub,
}
#[derive(Debug,Subcommand)]
enum BaseSub{
  #[
    command(
      name="config",
      about="config...."
    )
  ]cmd1(Cmd01),
  #[
    command(
      name="show",
      about="read info from Excel file."
    )
  ]cmd2(Cmd02),
}
#[derive(Debug,Args)]
struct Cmd01{
  #[command(subcommand)]
  cmd:Cmd01sub
}
#[derive(Debug,Args)]
struct Cmd02{
  #[command(subcommand)]
  cmd:Cmd02sub // option of enum, as args, for BaseSub::cmd1;
}
#[derive(Debug,Subcommand)]
enum Cmd01sub{
  #[
    command(
      name="new",
    )
  ]cmd1(Cmd0101),
}
#[derive(Debug,Subcommand)]
enum Cmd02sub{
  #[
    command(
      name="shape",
      about="show shape of sheets in the file.",
    )
  ]cmd1(Cmd0201),
  #[
    command(
      name="sht",
      about="show the whole expected sheet.",
    )
  ]cmd2(Cmd0202),
  #[
    command(
      name="row",
      about="show the target row data.",
    )
  ]cmd3(Cmd0203),
  #[
    command(
      name="col",
      about="show the target column data.",
    )
  ]cmd4(Cmd0204),
  #[
    command(
      name="match",
      about="match specific column by regular expression.",
    )
  ]cmd5(Cmd0205),
}
#[derive(Debug,Args)]
struct Cmd0101{
}
#[derive(Debug,Args)]
struct Cmd0201{
  #[arg(index=1,value_name="path",help="input file path")]
  ifp:String,
}
#[derive(Debug,Args)]
struct Cmd0202{
  #[arg(
    required=true,
    value_name="sheet name",
    help="sheet name"
  )]shtna:String,
  #[arg(
    required=true,
    value_name="path",
    help="input file path",
  )]ifp:String,
  #[arg(
    required=false,
    long="title",
    value_name="title",
    help="1-based index of the title row."
  )]title:Option<usize>,
}
#[derive(Debug,Args)]
struct Cmd0203{}
#[derive(Debug,Args)]
struct Cmd0204{}
#[derive(Debug,Args)]
struct Cmd0205{
  #[
    arg(
     required=true,
     short='r',
     long="mchl",
     value_name="regex_str,row_idx,col_idx",
     value_parser=super::psargs::parse_str_condition,
     help="match lines:filter conditons"
    )
  ]lines:Vec<(String,(usize,usize))>,
  #[
    arg(
     required=true,
     short='m',
     long="meta",
     value_name="shtna,ifp",
     value_parser=super::psargs::parse_sht,
     help="meta data to load Excels."
    )
  ]shtli:Vec<(String,String)>,
  #[
    arg(
      required=false,
      short='p',
      long="pretty",
      value_name="bool",
      help="if true, show in pretty table; else show the original data;"
    )
  ]pretty:Option<bool>,
}
pub fn run()->(){
  let cliargs=BaseCmd::parse();
  match cliargs.cmd{
    BaseSub::cmd1(_cmd)=>{ // config
      match _cmd.cmd{
        Cmd01sub::cmd1(_args)=>{ // config new
          todo!()
        },
        _=>{todo!()}
      }
    },
    BaseSub::cmd2(_cmd)=>{ // show
      match _cmd.cmd{
        Cmd02sub::cmd1(_args)=>{ // show shape
          println!(
            "{:?}",
          crate::brother::xlshow::get_shape(_args.ifp),
          );
        },
        Cmd02sub::cmd2(_args)=>{ // show sht
          let sht=crate::brother::ShtMeta::new(&_args.ifp,&_args.shtna);
          let shape:(usize,usize)=match crate::brother::get_sht_data(&sht){
            Ok(_range)=>{_range.get_size()},
            _=>{(0,0)}
          };
          match _args.title{
           Some(_title)=>{
             super::evince::term_show_table(
               crate::brother::xlshow::get_row(_title,&sht),
               crate::brother::xlshow::get_rows(((_title+1)..=shape.0).collect(),&sht),
             )
           },
           None=>{
             super::evince::term_show_rows(
               crate::brother::xlshow::get_rows((1..=shape.0).collect(),&sht)
             )
           },
          }
        },
        Cmd02sub::cmd5(_args)=>{ // show match
          let data=crate::brother::xlmatch::multi_sht_match(
          &_args.lines.iter().map(
            |_line|{crate::brother::StrMchLine::new(&_line.0,_line.1)}
          ).collect::<Vec<crate::brother::StrMchLine>>(),
          &_args.shtli.iter().map(
            |_sht|{crate::brother::ShtMeta::new(&_sht.0,&_sht.1)}
          ).collect::<Vec<crate::brother::ShtMeta>>(),
          );
          let if_pretty:bool=match _args.pretty{
            Some(_pretty)=>{_pretty},
            None=>{false}
          };
          if if_pretty==true{
            super::evince::term_show_multi_table(data);
          }else{
            for row in data{
              println!("{:?}",row);
            }
          }
        },
        _=>{todo!()}
      }
    },
  }
}
