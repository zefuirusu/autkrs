// use std::path::PathBuf;
use clap::{Parser,Subcommand,Args};

use crate::meta::MatchMeta;
use crate::brother::xlmatch::re_match_col;
use crate::brother::xlshow::{get_row,get_rows,get_shape};
use crate::cli::evince::term_show_table;
use crate::cli::psarg::{parse_shtmeta,parse_num_range};

#[derive(Debug,Parser)]
#[command(author="sk",name="autk",version="4.0.1", about="Auditors' Toolkit.", long_about = "A series of tools for auditors, who is struggling near the end of the year.")]
struct AUTK{
  #[command(subcommand)]
  basecmd:Lv1cmd,
}
// cmd lv1
#[derive(Debug,Subcommand)]
enum Lv1cmd{
  // new,config,show,meta,table,mgl
  // Config(ConfigCmd),
  #[
    command(
      name="show",
      about="read info from Excel file.",
    )
  ]Lv1sub2(ShowCmd),
  #[
    command(
      name="test",
      about="test for passing arguments."
    )
  ]Lv1test(TestCmd),
}
// cmd lv2
#[derive(Debug,Args)]
struct ShowCmd{
  #[command(subcommand)]
  subcmd:Showlv2cmd
}
#[derive(Debug,Args)]
struct TestCmd{
  #[command(subcommand)]
  subcmd:Testlv2cmd
}
#[derive(Debug,Subcommand)]
enum Testlv2cmd{
  #[
    command(
      name="lv2",
      about="lv2 cmd for test.",
      arg_required_else_help=true,
    )
  ]Tlv2C1(Tlv2C1Args),
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
  ]Row(RowArgs),
  #[
    command(
      name="shape",
      about="show shape of the Excel.",
      arg_required_else_help=true,
    )
  ]Shape(IfpArg)
}
// define args for cmd lv2;
#[derive(Debug,Args)]
struct Tlv2C1Args{
}
#[derive(Debug,Args)]
struct IfpArg{
  #[arg(index=1,value_name="string",help="input file path")]
  ifp:String,
}
#[derive(Debug,Args)]
struct ColMatchArgs{
  #[arg(short,long="regex",value_name="string",help="regular expression")]
  regex:String,
  #[arg(short,long="col",value_name="int",help="column number to match")]
  col:usize,
  #[arg(short,long="shtna",value_name="string",help="sheet name")]
  shtna:String,
  #[arg(short,long="title",value_name="int",default_value="1",help="row number of title of columns")]
  title:Option<usize>,
  #[arg(short,long="ifp",value_name="string",help="input file path")]
  ifp:String,
  // #[arg(short,long="save",help="save path of the output",default_value="out.xlsx")]
  // save:String,
  // #[arg(short,long="cal",value_name="string",help="apply calculation to the results, in the result column.",value_parser=["sum","average","count"],default_value="sum")]
  // cal:String,
}
#[derive(Debug,Clone,Args)]
struct RowArgs{
  #[arg(
      short='n',
      long="num",
      value_name="num",
      num_args=1..,
      value_delimiter=',',
      // value_parser=parse_num_range,
      help="index of row(s) to show."
  )]rows:Vec<usize>,
  #[arg(short,long="shtna",value_name="shtna",help="sheet name")]
  shtna:String,
  #[arg(short,long="ifp",value_name="ifp",help="input file path.")]
  ifp:String,
  #[arg(required=false,default_value="1",short,long="title",value_name="int",help="sheet name")]
  title:Option<usize>,
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
    Lv1cmd::Lv1sub2(_showcmd)=>{
      match _showcmd.subcmd{
        Showlv2cmd::Match(_colmatchargs)=>{
          term_show_table(
            get_row(
              _colmatchargs.title.clone(),
              _colmatchargs.ifp.clone(),
              _colmatchargs.shtna.clone()
            ),
            re_match_col(
              _colmatchargs.regex,
              _colmatchargs.col,
              _colmatchargs.shtna.clone(),
              _colmatchargs.ifp.clone(),
            ),
          );
        },
        Showlv2cmd::MultiMatch(_multimatchargs)=>{
          todo!()
        },
        Showlv2cmd::Row(_row_args)=>{
          term_show_table(
            get_row(
              _row_args.title,
              _row_args.clone().ifp,
              _row_args.clone().shtna,
            ),
            get_rows(
              _row_args.clone().rows,
              _row_args.clone().ifp,
              _row_args.clone().shtna,
            ),
          );
        },
        Showlv2cmd::Shape(_ifp)=>{
          println!("{:?}",get_shape(_ifp.ifp));
        },
      }
    },
    Lv1cmd::Lv1test(_testcmd)=>{
    },
  }
}
