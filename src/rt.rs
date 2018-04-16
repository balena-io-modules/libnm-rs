macro_rules! assert_initialized_main_thread {
    () => ()
}

macro_rules! callback_guard {
    () => (
        let _guard = ::glib::CallbackGuard::new();
        if cfg!(debug_assertions) {
            assert_initialized_main_thread!();
        }
    )
}
