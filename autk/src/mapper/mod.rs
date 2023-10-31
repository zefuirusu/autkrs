use std::collections::BTreeMap;
pub trait XlMap<'map>{
    // TODO
    fn show()->(){}
    fn append()->(){}
}
pub trait ChartMap<'map>:XlMap<'map>{
}
pub trait MglMap<'map>:XlMap<'map>{
    fn map_name()->&'map str{"mglmap"}
    fn key_name()->&'map str{"glid"}
    fn key_index()->Vec<&'map str>{vec!["date","jr",""]}
    fn accid_col()->&'map str{"accid"}
    fn accna_col()->&'map str{"accna"}
    fn drcrdesc()->Vec<&'map str>{vec!["dr","cr"]}
    fn top_accid_len()->i32{4}
    fn accna_split_by()->&'map str{"/"}
    fn date_col()->&'map str{"date"}
    fn date_split_by()->&'map str{"-"}
}
pub trait ApArMap<'map>:XlMap<'map>{}
pub trait InvMap<'map>:XlMap<'map>{}
