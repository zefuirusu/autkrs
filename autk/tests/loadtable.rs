use std::path::Path;
use calamine::{Reader, open_workbook, Xlsx, DataType};
use autk::funcs::check;
use autk::calculation::Acct;
use autk::calculation::Table;
use autk::calculation::ChartData;
use autk::brother::BkBro;

// const YLPATH:&'static str="./yl_rm_2021.xlsx";
const YLPATH:&'static str="/home/debvbx/Documents/autkrs/autk/tests/yl_rm_2021.xlsx";

#[test]
fn test_table_1()->(){
    let a1:Acct=Acct::new(
        "1002",
        "bank",
        true,
        1,
    );
    let b1:Table=Table::new();
    let c1:&ChartData=a1.get_amt(&b1);
    check(&a1);
    check(&b1);
    check(&c1);
}
#[test]
fn test_bro_1()->(){
    let mut workbook: Xlsx<_> = open_workbook(Path::new(YLPATH)).expect("Cannot open file");
    workbook.load_tables();
    // let workbook2: Xlsx<_> = open_workbook(Path::new(YLPATH)).expect("Cannot open file");
    // let shtna=open_workbook::<_,&Path>(Path::new(YLPATH)).expect("Cannot open file").table_names()[0];
    // println!("shtli:{:?}",shtna);
    if let Some(Ok(range)) = workbook.worksheet_range("inventory2020") {
        let total_cells = range.get_size().0 * range.get_size().1;
        let non_empty_cells: usize = range.used_cells().count();
        println!(
            "total cells:{:?};no empty cells:{:?}",total_cells,non_empty_cells
        );
        // assert_eq!(10,total_cells);
        // assert_eq!(10,non_empty_cells);
    }
}
