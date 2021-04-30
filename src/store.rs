use swipl::prelude::*;
use terminus_store::store::sync::*;

predicates! {
    pub semidet fn open_memory_store(_context, term) {
        let store = open_sync_memory_store();
        term.unify(&WrappedStore(store))
    }

    pub semidet fn open_directory_store(_context, dir_term, out_term) {
        let dir: String = dir_term.get()?;
        let store = open_sync_directory_store(&dir);
        out_term.unify(&WrappedStore(store))
    }
}

wrapped_clone_blob!("store", pub WrappedStore, SyncStore, defaults);
