use autk::cli::xlsch::re_match_col;
use autk::cli::cmd::run_autk;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to AUTK! current version 4.0.");
    run_autk();
    // match cli.command{
      // Commands::NewCmd=>{},
      // Commands::ConfigCmd=>{},
      // Commands::ShowCmd=>{},
      // Commands::MetaCmd=>{},
      // Commands::TableCmd=>{},
      // Commands::MglCmd=>{},
    // }
    // re_match_col(
      // "^使用权.*",//regex
      // "2024年1-6月序时账",//shtna
      // 5,//col_index,科目描述
      // "/home/debvbx/Documents/2024年1-6月序时账.xlsx",//file path
    // );
    // let args = Args::parse();
    // let re = Regex::new(&args.regex)?;
    // let col_index = args.col  - 1; // 转换为0-based索引
//
    // 打开Excel文件
    // let mut workbook: Xlsx<_> = open_workbook(args.ifp)?;
//
    // 获取指定工作表
    // let range = workbook
        // .worksheet_range(&args.shtna)
        // .ok_or("工作表不存在")??; //ok_or(), transform Option<T> into Result<T,E>;
//
    // 并行处理行数据
    // let matches: Vec<_> = range
        // .rows()
        // .par_bridge()
        // .filter_map(|row| {
            // let cell = row.get(col_index)?;
            // let text = match cell {
                // DataType::String(s) => s,
                // DataType::Float(f) => &f.to_string(),
                // DataType::Int(i) => &i.to_string(),
                // DataType::Bool(b) => if *b { "true" } else { "false" },
                // DataType::DateTime(f) => &f.to_string(),
                // DataType::Empty => "",
                // _ => "",
            // };
//
            // if re.is_match(text)  {
                // Some(text.to_string())
            // } else {
                // None
            // }
        // })
        // .collect();
//
    // 使用comfy_table展示结果
    // let mut table = Table::new();
    // table.set_header(vec![
        // Cell::new("行号").add_attribute(comfy_table::Attribute::Bold),
        // Cell::new("匹配内容").add_attribute(comfy_table::Attribute::Bold),
    // ]);
//
    // for (i, matched) in matches.iter().enumerate()  {
        // table.add_row(vec![(i  + 1).to_string(), matched.clone()]);
    // }
//
    // println!("{}", table);
    Ok(())
}
