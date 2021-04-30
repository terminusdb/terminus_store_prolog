use swipl::prelude::*;
use terminus_store::store::sync::*;
use std::sync::Arc;

predicates! {
    pub semidet fn open_memory_store(_context, term) {
        let store = open_sync_memory_store();
        term.unify(&WrappedStore(Arc::new(store)))
    }

    pub semidet fn open_directory_store(_context, dir_term, out_term) {
        let dir: String = dir_term.get()?;
        let store = open_sync_directory_store(&dir);
        out_term.unify(&WrappedStore(Arc::new(store)))
    }
}

wrapped_arc_blob!(pub "store", WrappedStore, SyncStore, defaults);
