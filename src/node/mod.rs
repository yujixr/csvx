mod add;
mod bitwise_and;
mod bitwise_or;
mod div;
mod equal;
mod fn_acos;
mod fn_acosh;
mod fn_asin;
mod fn_asinh;
mod fn_atan;
mod fn_atanh;
mod fn_avg;
mod fn_ceil;
mod fn_cos;
mod fn_cosh;
mod fn_floor;
mod fn_if;
mod fn_ln;
mod fn_log;
mod fn_log10;
mod fn_log2;
mod fn_pow;
mod fn_ref;
mod fn_round;
mod fn_sin;
mod fn_sinh;
mod fn_sqrt;
mod fn_sum;
mod fn_tan;
mod fn_tanh;
mod greater_than;
mod greater_than_equal;
mod left_shift;
mod less_than;
mod less_than_equal;
mod logical_and;
mod logical_or;
mod minus;
mod modulo;
mod mul;
mod not;
mod not_equal;
mod parse;
mod right_shift;
mod sub;
mod value;
mod xor;

use super::token::*;
pub use parse::parse;
pub use value::Value;

pub type ThreadSafeNode = dyn Node + Sync + Send;

pub trait Node {
    fn new(seqs: Vec<Vec<Token>>) -> (Box<ThreadSafeNode>, Vec<(usize, usize)>)
    where
        Self: Sized + Sync + Send;
    fn calc(&self, calculated_table: &Vec<Vec<Value>>) -> Value;
}
