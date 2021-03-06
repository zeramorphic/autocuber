// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
pub(crate) use log;

macro_rules! dbg2 {
    ( $e:expr ) => {
        match $e {
            e => {
                web_sys::console::log_1(&format!("{:#?}", e).into());
                e
            }
        }
    };
}
pub(crate) use dbg2;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    {
        console_error_panic_hook::set_once();
        log!("set panic hook");
    }
    #[cfg(not(feature = "console_error_panic_hook"))]
    {
        log!("did not set panic hook (feature console_error_panic_hook not enabled)");
    }
}
