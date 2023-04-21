fn divmod(dividend: i32, divisor: i32) -> (i32, i32) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)
}

#[derive(Debug, Clone)]
pub struct Tick {
    pub value: i32
}

impl Tick {
    pub fn new() -> Self {
        Self {
            value: 0,
        }
    }

    pub fn datetime(&self) -> (i32, i32, i32) {
        let (years, temp) = divmod(self.value as i32, 360);
        let (months, days) = divmod(temp, 30);

        (years, months, days)
    }
}
