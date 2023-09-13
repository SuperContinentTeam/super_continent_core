use lazy_static::lazy_static;
use rand::{distributions::WeightedIndex, prelude::Distribution};

lazy_static! {
    pub static ref ENVIRONMENT_TYPES: [i32; 5] = [-2, -1, 0, 1, 2];
    pub static ref WI: WeightedIndex<i32> = WeightedIndex::new([6, 20, 51, 15, 4].iter()).unwrap();
}

pub fn random_block_env() -> i32 {
    let mut rng = rand::thread_rng();
    let v = ENVIRONMENT_TYPES[WI.sample(&mut rng)];
    v
}

pub fn format_number(number: i32, monthly: Option<bool>) -> String {
    let abs_number = number.abs();
    let neg = number < 0;
    let b_monthly = if monthly.is_none() {
        false
    } else {
        monthly.unwrap()
    };

    "MAX".to_string()
}

// def format_number(number, monthly=False):
//     abs_number = abs(number)
//     pre = "-" if number < 0 else ("+" if monthly else "")

//     if abs_number < 1000:
//         result = abs_number
//     elif 1000 <= abs_number < 1000000:
//         result = str(round(abs_number / 1000, 2)) + "K"
//     elif 1000000 <= abs_number < 1000000000:
//         result = str(round(abs_number / 1000000, 2)) + "M"
//     elif 1000000000 <= abs_number < 1000000000000:
//         result = str(round(abs_number / 1000000000, 2)) + "G"
//     else:
//         result = "MAX"

//     return f"{pre}{result}"
