use terminus_store_prolog_core;

#[no_mangle]
pub extern "C" fn install() {
    terminus_store_prolog_core::install(None);
}
