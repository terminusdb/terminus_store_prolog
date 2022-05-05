use terminus_store_prolog;

#[no_mangle]
pub extern "C" fn install() {
    terminus_store_prolog::install(None);
}
