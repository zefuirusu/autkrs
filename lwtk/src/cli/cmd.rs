use clap::{Parser,Subcommand,Args};

use crate::brother::{ShtMeta,StrMchLine};

#[derive(Debug,Parser)]
#[
  command(
    author="sk",
    name="lwtk",
    version="1.0.1",
    about="AUTK lightweight version.",
    long_about="",
  )
]struct BaseCmd{
  #[command(subcommand)]
  cmd:Lv1option,
}
#[derive(Debug,Subcommand)]
enum Lv1option{
  #[
    command(
      name="show",
      about="read info from Excel file."
    )
  ]cmd1(Lv1cmd1),
  #[
    command(
      name="config",
      about="config...."
    )
  ]cmd2(Lv1cmd2),
}
#[derive(Debug,Args)]
struct Lv1cmd2{
  #[command(subcommand)]
  cmd:Lv2option
}
#[derive(Debug,Args)]
struct Lv1cmd1{
  #[command(subcommand)]
  cmd:Lv2option // option of enum, as args, for Lv1option::cmd1;
}
#[derive(Debug,Subcommand)]
enum Lv2option{
  #[
    command(
      name="shape",
    )
  ]cmd1(Lv2cmd1),
  #[
    command(
      name="match",
      about="",
    )
  ]cmd2(Lv2cmd2),
}
#[derive(Debug,Args)]
struct Lv2cmd1{}
#[derive(Debug,Args)]
struct Lv2cmd2{
  // #[
    // arg(
      // required=true,
      // short='i',
      // long="mchl",
      // value_name="condition",
      // value_parser="",
      // help="match lines:filter conditons"
    // )
  // ]lines:Vec<StrMchLine>,
  // #[
    // arg(
      // required=true,
      // short='m',
      // long="meta",
      // value_name="sheet meta",
      // value_parser="",
      // help="meta data to load Excels."
    // )
  // ]shtli:Vec<ShtMeta>
}
pub fn run()->(){
  let cliargs=BaseCmd::parse();
  match cliargs.cmd{
    Lv1option::cmd1(_cmd)=>{
      match _cmd.cmd{
        Lv2option::cmd1(_cmd)=>{
          todo!()
        },
        Lv2option::cmd2(_cmd)=>{
          todo!()
        },
      }
    },
    Lv1option::cmd2(_cmd)=>{
      todo!()
    },
  }
}
