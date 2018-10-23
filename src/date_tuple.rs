pub struct DateTuple {
    y: u32,
    m: u32,
    d: u32,
}

impl DateTuple {
    pub fn new(y: u32, m: u32, d: u32) -> DateTuple {
        DateTuple { y, m, d }
    }

    pub fn get_year(&self) -> u32 {
        self.y
    }

    pub fn get_month(&self) -> u32 {
        self.m
    }

    pub fn get_date(&self) -> u32 {
        self.d
    }
}

#[cfg(test)]
mod tests {

    //todo toString
    //todo equals
    //todo compareTo

}
