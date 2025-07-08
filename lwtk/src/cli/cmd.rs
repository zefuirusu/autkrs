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
    )
  ]cmd1(Cmd0201),
  #[
    command(
      name="sht",
      about="",
    )
  ]cmd2(Cmd0202),
  #[
    command(
      name="row",
      about="",
    )
  ]cmd3(Cmd0203),
  #[
    command(
      name="col",
      about="",
    )
  ]cmd4(Cmd0204),
  #[
    command(
      name="match",
      about="",
    )
  ]cmd5(Cmd0205),
}
#[derive(Debug,Args)]
struct Cmd0101{}
#[derive(Debug,Args)]
struct Cmd0201{}
#[derive(Debug,Args)]
struct Cmd0202{}
#[derive(Debug,Args)]
struct Cmd0203{}
#[derive(Debug,Args)]
struct Cmd0204{}
#[derive(Debug,Args)]
struct Cmd0205{
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
    BaseSub::cmd1(_cmd)=>{
      match _cmd.cmd{
        Cmd01sub::cmd1(_args)=>{todo!()},
        _=>{todo!()}
      }
    },
    BaseSub::cmd2(_cmd)=>{
      match _cmd.cmd{
        Cmd02sub::cmd5(_args)=>{todo!()},
        _=>{todo!()}
      }
    },
  }
}
