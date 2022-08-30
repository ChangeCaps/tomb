#[macro_export]
macro_rules! log {
    ($($tt:tt)*) => {
        ::web_sys::console::log_1(&::std::convert::From::from(::std::format!($($tt)*)));
    };
}
