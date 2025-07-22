use comfy_table::{Cell,Table,ContentArrangement};
use rayon::prelude::*;

fn get_prepared_table()->Table{
    let mut table:Table=Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    return table;
}
fn load_rows<'show>(
    table:&'show mut Table,
    rows:&'show Vec<Vec<String>>
)->(){
    for row in rows.iter(){
        table.add_row(row);
    }
}
pub fn term_show_rows<'show>(
    rows:&'show Vec<Vec<String>>
)->()
{
    let mut table:Table=get_prepared_table();
    table.set_header(
        (1..=rows[0].len()).map(
            |title_index|{
                Cell::new(title_index)
                    .add_attribute(comfy_table::Attribute::Bold)
            }
        )
    );
    load_rows(&mut table,rows);
    println!("{}",&table);
}
pub fn term_show_table<'show>(
    title:&'show Vec<String>,
    rows:&'show Vec<Vec<String>>,
)
->()
{
    let mut table:Table=get_prepared_table();
    table.set_header(
        title.into_iter().map(
            |string_value|{
                Cell::new(string_value)
                  .add_attribute(comfy_table::Attribute::Bold)
            }
        )
    );
    load_rows(&mut table,rows);
    println!("{}",table);
}
fn row_from_multi_table(
    lines:Vec<(String,(usize,usize))>,
    shtli:Vec<(String,String)>,
)->Vec<Vec<String>>{
    // TODO brother::xlshow::get_title_from_tables;
    todo!()
}
pub fn term_show_multi_table<'show>(
    rows:&Vec<Vec<String>>,
)->(){
    /*
    TODO
    show tables independently;
    */
    term_show_rows(rows);
}
#[test]
pub fn try_show_table()->(){
  let title:Vec<String>=vec!["a".to_string(),"b".to_string(),"c".to_string()];
  let data:Vec<Vec<String>>=vec![
    vec![12.to_string(),4.to_string(),6.to_string()],
    vec![15.to_string(),8.to_string(),22.to_string()],
    vec![215.to_string(),88.to_string(),22.to_string()],
  ];
  term_show_table(&title,&data);
}
