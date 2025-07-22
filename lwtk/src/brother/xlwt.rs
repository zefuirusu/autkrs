use std::path::{Path};
use std::fs::File;
use rust_xlsxwriter::{Workbook,Worksheet};
use super::ShtMeta;

pub fn rgstr2xl<'save>(
    data:&'save Vec<Vec<String>>,
    save_sht:&'save ShtMeta,
)->(){
    /*
    save Range<T> when `T` is `String` into `*.xlsx` file;
    */
    let path:&Path=Path::new(save_sht.ifp.as_str());
    let file:File=match path.is_file(){
        true=>{
            println!("Warning! crate `rust_xlsxwriter` cannot write to LOCAL(existed) Excel file currently!\nWait longer for the developer to enhance this crate.");
            File::open
        },
        false=>{File::create},
    }(save_sht.ifp.as_str())
    .expect("Invalid path!");
    let mut wb:Workbook=Workbook::new();
    let mut ws:&mut Worksheet=wb.add_worksheet_with_low_memory();
    ws.set_name(save_sht.shtna.as_str());
    ws.write_row_matrix(
        0,0,
        vec![
            (1..=data[0].len()).map(
                |x|{x.to_string()}
            ).collect::<Vec<String>>()
        ],
    );
    ws.write_row_matrix(1,0,data);
    wb.save_to_writer(file)
        .expect("failed to write!");
    println!(
        "save to new file:{:?},\n\tsheet name:{}",
        &path,save_sht.shtna,
    );
}
