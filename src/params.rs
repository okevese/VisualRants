
#[derive(Debug)]
pub enum Sort {}

impl Sort {
    pub const RECENT: &'static str = "recent";
    pub const TOP: &'static str = "top";
    pub const ALGO: &'static str = "algo";
}


pub enum Range {}

impl Range {
    pub const DAY: &'static str = "day";
    pub const WEEK: &'static str = "week";
    pub const MONTH: &'static str = "month";
    pub const ALL: &'static str = "all";
}

