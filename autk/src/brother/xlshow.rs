use rayon::prelude::*;
use calamine::{open_workbook, Range, Reader, Xls, Xlsx};
use std::collections::HashMap;
use std::path::Path;

use crate::brother::{get_sht_data, value2str, ShtMeta};

pub fn get_shape(ifp: &String) -> HashMap<String, Vec<usize>> {
    //TODO: This function does not run at multi-thread by rayon currently;
    //TODO: This function cannot distinguish `xlsx` or `xls`;
    // match extsn(ifp.as_str()){
    // Some(_any_ext)=>{
    // match _any_ext{
    // "xlsx"=>{
    // let mut wb:Xlsx<_>=open_workbook(Path::new(ifp.as_str()))
    // .expect("Cannot open Excel! Invalid file path!");
    // let shape=wb.sheet_names().iter().map(
    // |sht|{
    // let shtshape=wb.worksheet_range(sht.as_str()).expect("").get_size();
    // (sht,vec![shtshape.0,shtshape.1])
    // }
    // ).collect::<HashMap<String,Vec<usize>>>();
    // Some(shape)
    // },
    // "xls"=>{
    // let mut wb:Xls<_>=open_workbook(Path::new(ifp.as_str()))
    // .expect("Cannot open Excel! Invalid file path!");
    // let shape=wb.sheet_names().iter().map(
    // |sht|{
    // let shtshape=wb.worksheet_range(sht.as_str()).expect("").get_size();
    // (sht,vec![shtshape.0,shtshape.1])
    // }
    // ).collect::<HashMap<String,Vec<usize>>>();
    // Some(shape)
    // },
    // _=>{None}
    // }
    // },
    // None=>{None},
    // }
    let mut shape: HashMap<String, Vec<usize>> = HashMap::new();
    let mut wb: Xlsx<_> =
        open_workbook(Path::new(ifp))
            .expect("Cannot open Excel! Invalid file path!");
    for shtna in wb.sheet_names() {
        let range: Range<_> = wb.worksheet_range(&shtna)
            .expect("Cannot get range!");
        shape.insert(
            shtna.to_string(),
            Vec::from([range.get_size().0, range.get_size().1]),
        );
    }
    shape
}
pub fn get_row<'xl>(
    _row_index: usize, // 1-based index.starts from 1;
    sht: &'xl ShtMeta,
) -> Vec<String> {
    get_sht_data(sht)
        .expect("failed to get sheet range!")
        .rows()
        .nth(_row_index - 1)
        .expect("Fail to the this row!")
        .into_par_iter()
        .map(|cell_value| value2str(cell_value))
        .collect()
}
pub fn get_rows<'xl>(
    rows_index: Vec<usize>,
    sht: &'xl ShtMeta
) -> Vec<Vec<String>> {
    rows_index
        .into_par_iter()
        .map(|row_index| get_row(row_index, sht))
        .collect()
}
pub fn get_col(
    // col_index:usize
) -> Vec<String> {
    todo!()
}
pub fn get_title_from_tables() -> Vec<Vec<String>> {
    todo!()
}
