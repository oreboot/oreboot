pub fn is_aligned(x: u32, a: u32) -> bool {
    (x & (a - 1)) == 0
}

pub fn align_down(x: u32, a: u32) -> u32 {
    x & !(a - 1)
}

/* FIXME: currently doesn't need variadic, rework again as macro when it does
/// Helper macro to retry until a condition becomes true or the maximum number
/// of attempts is reached. Two forms are supported:
///
/// 1. retry(attempts, condition)
/// 2. retry(attempts, condition, expr)
///
/// @param attempts	Maximum attempts.
/// @param condition	Condition to retry for.
/// @param expr		Procedure to run between each evaluation to "condition".
///
/// @return Condition value if it evaluates to true within the maximum attempts;
///	   0 otherwise.
#[macro_export]
macro_rules! retry {
    ($attempts:expr, $condition:expr, $wait_fn:ident, $wait_arg:expr) => {
    };
}
*/

pub fn retry(attempts: u32, condition: u32, wait_fn: fn(u32), wait_arg: u32) -> u32 {
    let mut retry_attempts = attempts;
    let retry_ret = condition;
    loop {
        if retry_ret != 0 {
            break;
        }
        retry_attempts -= 1;
        if retry_attempts > 0 {
            wait_fn(wait_arg);
        } else {
            break;
        }
    }
    retry_ret
}
