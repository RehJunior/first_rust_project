pub struct City {
    pub description: String,
    pub residents: u64,
    pub is_coastal: bool,
}

pub fn new_city(residents: u64, is_coastal: bool) -> City {
    if is_coastal {
        City {
            description: format!("a *coastal* city of approximately {} residents", residents),
            residents,
            is_coastal,
        }
    } else {
        City {
            description: format!(
                "a *non-coastal* city of approximately {} residents",
                residents
            ),
            residents,
            is_coastal,
        }
    }
}
