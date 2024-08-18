use log::Level;
use pushkind_store::app::App;
use tracing_subscriber::fmt;
use tracing_subscriber_wasm::MakeConsoleWriter;

fn main() {
    fmt()
        .with_writer(MakeConsoleWriter::default().map_trace_level_to(tracing::Level::DEBUG))
        .without_time()
        .init();

    // _ = console_log::init_with_level(Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App)
}
