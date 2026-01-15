use std::path::Path;
use calamine::{open_workbook,Xlsx};

use crate::brother::BkBro;
use crate::mapper::XlMap;
impl<'bk> BkBro{
    pub fn load_from_path(self:&mut Self,path:&'bk str)->()
    {
        let mut bk:Xlsx<_>=open_workbook(Path::new(path)).expect("cannot open file.");
        // bk.load_tables();
        // println!("{:?}",bk.sheet_names());
    }
    pub fn shtli()->(){}
    pub fn shape()->(usize,usize){(0,0)}
    pub fn test_map(self:&Self,xlmap:impl XlMap<'bk>,common_title:u8)->bool{false}
    pub fn find_sheet(regex_str:&'static str)->(){}
    pub fn get_sheet(sheet_name:&'static str)->(){}
    pub fn get_value()->(){}
    pub fn get_matrix()->(){}
    pub fn search()->(){}
}
