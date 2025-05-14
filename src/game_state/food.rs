pub enum Food {
    Grass,
    Apple,
}

impl Food {
    pub fn num_bites(&self) -> u32 {
        match self {
            Food::Grass => 3,
            Food::Apple => 5,
        }
    }

    pub fn xp_value(&self) -> u128 {
        match self {
            Food::Grass => 10,
            Food::Apple => 50,
        }
    }
}
