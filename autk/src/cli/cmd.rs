// use std::path::PathBuf;
use clap::{Parser,Subcommand,Args};

use crate::meta::MatchMeta;
use crate::brother::xlsch::re_match_col;
use crate::brother::xlshow::{get_row,get_rows};
use crate::cli::evince::term_show_table;

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
      name="match",
      alias="search",
      about="match values of a specific column by regular expression.",
      arg_required_else_help=true
    )
  ]Match(ColMatchArgs),
  #[
    command(
      name="multimatch",
      alias="mmch",
      about="multi-workbooks ,match values of a specific column by regular expression.",
      arg_required_else_help=true
    )
  ]MultiMatch(MultiMatchArgs),
  #[
    command(
      name="row",
      about="show one or several rows.",
      arg_required_else_help=true,
    )
  ]Row(RowArgs)
}
#[derive(Debug,Args)]
struct ColMatchArgs{
  #[arg(short,long="regex",value_name="string",help="regular expression")]
  regex:String,
  #[arg(short,long="col",value_name="int",help="column number to match")]
  col:usize,
  #[arg(short,long="resu",value_name="int",num_args=1,value_delimiter=',',default_value="0",help="number of the result column")]
  resu:usize,
  #[arg(short,long="shtna",value_name="string",help="sheet name")]
  shtna:String,
  #[arg(short,long="title",value_name="int",default_value="1",help="row number of title of columns")]
  title:usize,
  #[arg(short,long="ifp",value_name="string",help="input file path")]
  ifp:String,
  // #[arg(short,long="save",help="save path of the output",default_value="out.xlsx")]
  // save:String,
  // #[arg(short,long="cal",value_name="string",help="apply calculation to the results, in the result column.",value_parser=["sum","average","count"],default_value="sum")]
  // cal:String,
}
#[derive(Debug,Clone,Args)]
struct RowArgs{
  #[arg(default_value="1",short='n',long="num",value_name="int",num_args=1..,help="row(s) to show.")]
  row:Vec<usize>,
  #[arg(short,long="shtna",value_name="string",help="sheet name")]
  shtna:String,
  #[arg(short,long="ifp",value_name="string",help="input file path.")]
  ifp:String,
  #[arg(required=false,default_value="1",short,long="title",value_name="int",help="sheet name")]
  title:usize,
}
fn parse_shtmeta(
  single_meta:&str,
)->Result<MatchMeta,String>{
  todo!()
}
#[derive(Debug,Args)]
struct MultiMatchArgs{
  #[arg(short,long="regex",help="regular expression")]
  regex:String,
  #[arg(short,long="meta",help="",num_args=1..,value_parser=parse_shtmeta)]
  meta:Vec<MatchMeta>,
}
pub fn run_autk()->(){
  let cliargs=AUTK::parse();
  match cliargs.basecmd{
    Lv1cmd::Show(_showcmd)=>{
      match _showcmd.subcmd{
        Showlv2cmd::Match(_colmatchargs)=>{
          re_match_col(
            _colmatchargs.regex,
            _colmatchargs.resu,
            _colmatchargs.col,
            _colmatchargs.shtna,
            _colmatchargs.title,
            _colmatchargs.ifp,
          );
        },
        Showlv2cmd::MultiMatch(_multimatchargs)=>{
          todo!()
        },
        Showlv2cmd::Row(_row_args)=>{
          term_show_table(
            get_row(_row_args.title,_row_args.clone().ifp,_row_args.clone().shtna),
            get_rows(_row_args.clone().row,_row_args.clone().ifp,_row_args.clone().shtna),
          );
        },
      }
    },
  }
}
