use clap::{Parser,Subcommand,Args};

#[derive(Debug,Parser)]
#[
    command(
        author="Skandha",
        name="autk",
        version="0.4.0",
        about="AUTK",
        long_about="Auditors' Toolkit.",
    )
]struct BaseCmd{
    #[command(subcommand)]
    cmd:BaseSub,
}
#[derive(Debug,Subcommand)]
enum BaseSub{
    #[command(
        name="config",
        about="config...."
    )]Cmd1(Cmd01),
    #[command(
        name="show",
        about="read info from Excel file."
    )]Cmd2(Cmd02),
}
#[derive(Debug,Args)]
struct Cmd01{
    #[command(subcommand)]
    cmd:Cmd01sub
}
#[derive(Debug,Args)]
struct Cmd02{
    #[command(subcommand)]
    cmd:Cmd02sub // option of enum, as args, for BaseSub::Cmd1;
}
#[derive(Debug,Subcommand)]
enum Cmd01sub{
    #[command(
      name="new",
    )
    ]Cmd1(Cmd0101),
}
#[derive(Debug,Subcommand)]
enum Cmd02sub{
    #[command(
        name="shape",
        about="show shape of sheets in the file.",
    )]Cmd1(Cmd0201),
    #[command(
        name="sht",
        about="show the whole expected sheet.",
    )]Cmd2(Cmd0202),
    #[command(
        name="row",
        about="show the target row data.",
    )]Cmd3(Cmd0203),
    #[command(
        name="col",
        about="show the target column data.",
    )
    ]Cmd4(Cmd0204),
    #[command(
        name="match",
        about="match specific column by regular expression.",
    )]Cmd5(Cmd0205),
}
#[derive(Debug,Args)]
struct Cmd0101{
}
#[derive(Debug,Args)]
struct Cmd0201{ // show shape
    #[arg(
        index=1,
        value_name="path",
        help="input file path"
    )]ifp:Vec<String>,
}
#[derive(Debug,Args)]
struct Cmd0202{ // show sht
    #[arg(
        required=true,
        short='m',
        long="meta",
        value_parser=super::psargs::parse_sht,
        value_name="string,string",
        help="<ifp,shtna>:meta data to load Excels"
    )]shtli:Vec<(String,String)>, //(ifp,shtna),
    #[arg(
        required=false,
        short='t',
        long="title",
        value_name="int",
        help="1-based row index of the title."
    )]title:Option<usize>,
}
#[derive(Debug,Args)]
struct Cmd0203{ // show row
    #[arg(
        required=true,
        short='n',
        long="num",
        value_name="int",
        num_args=1..,
        value_delimiter=',',
        // value_parser=super::psargs::parse_range,
        help="1-based row index",
    )]num:Vec<usize>,
    #[arg(
        required=true,
        short='m',
        long="meta",
        value_name="string,string",
        value_parser=super::psargs::parse_sht,
        help="<ifp,shtna>:meta data to load Excels:"
    )]shtli:(String,String), //(ifp,shtna),
    #[arg(
        required=false,
        short='t',
        long="title",
        value_name="int",
        help="1-based row index for the title.",
    )]title:Option<usize>,
}
#[derive(Debug,Args)]
struct Cmd0204{}
#[derive(Debug,Args)]
struct Cmd0205{ // show match
    #[arg(
        required=true,
        short='r',
        long="mchl",
        value_parser=super::psargs::parse_str_condition,
        value_name="int,int,string",
        help="<rdx,cdx,regex>:match lines, filter conditons"
    )]lines:Vec<(usize,usize,String)>,// (rdx,cdx,regex_str)
    #[arg(
        required=true,
        short='m',
        long="meta",
        value_parser=super::psargs::parse_sht,
        value_name="string,string",
        help="<ifp,shtna>:meta data to load Excels"
    )]shtli:Vec<(String,String)>,// (ifp,shtna)
    #[arg(
        required=false,
        short='o',
        long="save",
        value_parser=super::psargs::parse_sht,
        value_name="string,string",
        help="<ifp,shtna>:save path and sheet name for the output data",
    )]save:Option<(String,String)>,// (ifp,shtna)
    #[arg(
        required=false,
        short='p',
        long="pretty",
        value_name="bool",
        help="if true, show in pretty table; else show the original data;"
    )]pretty:Option<bool>,
}
pub fn run()->(){
  let cliargs=BaseCmd::parse();
  match cliargs.cmd{
    BaseSub::Cmd1(_cmd)=>{ // config
      match _cmd.cmd{
        Cmd01sub::Cmd1(_args)=>{ // config new
            todo!()
        },
        // _=>{todo!()}
      }
    },
    BaseSub::Cmd2(_cmd)=>{ // show
      match _cmd.cmd{
        Cmd02sub::Cmd1(_args)=>{ // show shape
            for _ifp in _args.ifp.iter(){
                let _shape_map:std::collections::BTreeMap<String,Vec<usize>>=std::collections::BTreeMap::from_iter(crate::brother::xlshow::get_shape(_ifp));
                let _shape_vec:Vec<Vec<String>>=_shape_map.into_iter().map(
                    |(_shtna,_shape)|{
                        vec![_shtna,_shape[0].to_string(),_shape[1].to_string()]
                    }
                ).collect::<Vec<Vec<String>>>();
                println!("{:?}:",_ifp,);
                super::evince::term_show_table(
                    &Vec::from(["sheet","rows","columns"])
                        .into_iter()
                        .map(|x|x.to_string())
                        .collect(),
                    &_shape_vec
                );
            }
        },
        Cmd02sub::Cmd2(_args)=>{ // show sht
            for _sht in _args.shtli.iter(){
                let sht=crate::brother::ShtMeta::from(_sht);
                let shape:(usize,usize)=match crate::brother::get_sht_data(&sht){
                    Ok(_range)=>{
                        let shape:(usize,usize)=_range.get_size();
                        println!(
                            "shape:{:?},sht:{:?}",
                            &shape,
                            &sht,
                        );
                        shape
                    },
                    _=>{println!("blank sheet!{:?}",&sht);(0usize,0usize)}
                };
                match _args.title{
                    Some(_title)=>{
                        super::evince::term_show_table(
                            &crate::brother::xlshow::get_row(_title,&sht),
                            &crate::brother::xlshow::get_rows(
                                ((_title+1)..=shape.0).collect(),
                                &sht,
                            ),
                        )
                    },
                    None=>{
                        super::evince::term_show_rows(
                            &crate::brother::xlshow::get_rows(
                                (1..=shape.0).collect::<Vec<usize>>(),
                                &sht,
                            )
                        );
                    },
                }
            }
            // let sht=crate::brother::ShtMeta::new(&_args.shtna,&_args.ifp);
            // let shape:(usize,usize)=match crate::brother::get_sht_data(&sht){
            //     Ok(_range)=>{
            //         let shape:(usize,usize)=_range.get_size();
            //         println!(
            //             "shape:{:?},name:{:?},path:{:?}",
            //             &shape,
            //             &_args.shtna,
            //             &_args.ifp,
            //         );
            //         shape
            //     },
            //     _=>{println!("blank sheet!");(0,0)}
            // };
        },
        Cmd02sub::Cmd3(_args)=>{ // show row
            let sht=crate::brother::ShtMeta::from(&_args.shtli);
            match _args.title{
                Some(_title)=>{
                    super::evince::term_show_table(
                        &crate::brother::xlshow::get_row(
                            _title,
                            &sht,
                        ),
                        &crate::brother::xlshow::get_rows(
                            _args.num,
                            &sht,
                        ),
                    );
                },
                None=>{
                    super::evince::term_show_rows(
                        &crate::brother::xlshow::get_rows(
                            _args.num,
                            &sht,
                        )
                    );
                },
            }
        },
        Cmd02sub::Cmd5(_args)=>{ // show match
            let data=crate::brother::xlmatch::multi_sht_match(
                &_args.lines.iter().map(
                    |_line|{crate::brother::StrMchLine::from(_line)}
                ).collect::<Vec<crate::brother::StrMchLine>>(),
                &_args.shtli.iter().map(
                    |_sht|{crate::brother::ShtMeta::from(_sht)}
                ).collect::<Vec<crate::brother::ShtMeta>>(),
            );
            let if_pretty:bool=match _args.pretty{
                Some(_pretty)=>{_pretty},
                None=>{false}
            };
            if if_pretty==true{
                super::evince::term_show_multi_table(&data);
            }else{
                for row in &data{
                    println!("{:?}",row);
                }
            }
            match _args.save{
                Some(_sht)=>{
                    crate::brother::xlwt::rgstr2xl(
                        &data,
                        &crate::brother::ShtMeta::from(&_sht)
                    )
                },
                None=>{},
            }
        },
        _=>{todo!()}
      }
    },
  }
}
