#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        use web_sys::console;
        console::log_1(&format!( $( $t )* ).into());
    }
}
