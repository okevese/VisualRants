
pub enum Sort {
    Algo,
    Top,
    Recent,
}


pub enum Range {
    Day,
    Week,
    Month,
    All,
}

// To ensure only correct string values for `get_rants` parameters
impl Sort {
    pub fn as_str(&self) -> &str {
        match self {
            &Sort::Algo => "algo",
            &Sort::Top => "top",
            &Sort::Recent => "recent",
        }
    }
}


impl Range {
    pub fn as_str(&self) -> &str {
        match self {
            &Range::Day => "day",
            &Range::Week => "week",
            &Range::Month => "month",
            &Range::All => "all",
        }
    }
}