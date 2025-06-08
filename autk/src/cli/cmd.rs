// use std::path::PathBuf;
use clap::{Parser,Subcommand,Args};

use crate::cli::xlsch::re_match_col;

#[derive(Debug,Parser)]
#[command(author, name="autk",version="4.0", about="Auditors' Toolkit.", long_about = None)]
struct AUTK{
  #[command(subcommand)]
  basecmd:Lv1cmd,
}
// cmd lv1
#[derive(Debug,Subcommand)]
enum Lv1cmd{
  // new,config,show,meta,table,mgl
  // #[command(subcommand)]
  // Config(ConfigCmd),
  Show(ShowCmd),
}
// cmd lv2
#[derive(Debug,Args)]
struct ShowCmd{
  #[command(subcommand)]
  subcmd:Showlv2cmd
}
#[derive(Debug,Subcommand)]
enum Showlv2cmd{
  #[
    command(
      name="search",
      alias="match",
      about="match values of a specific column by regular expression.",
      arg_required_else_help=true
    )
  ]Search(ColMatchArgs),
}
#[derive(Debug,Args)]
struct ColMatchArgs{
  #[arg(short,long="regex",help="regular expression")]
  regex:String,
  #[arg(short,long="shtna",help="sheet name")]
  shtna:String,
  #[arg(short,long="col",help="column number to match")]
  col:usize,
  #[arg(short,long="ifp",help="input file path")]
  ifp:String,
  #[arg(short,long="title",help="row number of title of columns",default_value="1")]
  title:usize,
}
pub fn run_autk()->(){
  let cliargs=AUTK::parse();
  match cliargs.basecmd{
    Lv1cmd::Show(_showcmd)=>{
      match _showcmd.subcmd{
        Showlv2cmd::Search(_colmatchargs)=>{
          re_match_col(
            _colmatchargs.regex,
            _colmatchargs.shtna,
            _colmatchargs.col,
            _colmatchargs.title,
            _colmatchargs.ifp,
          );
        },
      }
    },
  }
}
