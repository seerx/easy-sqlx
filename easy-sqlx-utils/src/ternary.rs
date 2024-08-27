// export_macro!
// 定义宏
#[macro_export]
macro_rules! ternary {
    ($condition:expr, $if_true:expr, $if_false:expr) => {
        if $condition {
            $if_true
        } else {
            $if_false
        }
    };
}