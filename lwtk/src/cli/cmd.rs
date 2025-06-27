use clap::{Parser,Subcommand,Args};

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
struct Lv2cmd2{}
