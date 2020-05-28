pub const INITIATE: i32 = 0;
pub const COUNTER_VALUE: i32 = 1;
pub const INCREASE_COUNTER: i32 = 2;
pub const DECREASE_COUNTER: i32 = 3;
pub const ERROR: i32 = 4;
pub const SNACK_BAR_MSG: i32 = 5;

pub fn to_string(what: i32) -> &'static str {
    match what {
        INITIATE => "INITIATE",
        COUNTER_VALUE => "COUNTER_VALUE",
        INCREASE_COUNTER => "INCREASE_COUNTER",
        DECREASE_COUNTER => "DECREASE_COUNTER",
        ERROR => "ERROR",
        SNACK_BAR_MSG => "SNACK_BAR_MSG",
        _ => "<UNKNOWN>",
    }
}
