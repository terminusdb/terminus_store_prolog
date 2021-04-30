use crate::builder::*;
use crate::layer::*;
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

    pub semidet fn open_write(context, store_or_layer_term, builder_term) {
        let builder;
        if let Some(store) = attempt_opt(store_or_layer_term.get::<WrappedStore>())? {
            builder = context.try_or_die(store.create_base_layer())?;
        }
        else {
            let layer: WrappedLayer = store_or_layer_term.get()?;
            builder = context.try_or_die(layer.open_write())?;
        }

        builder_term.unify(WrappedBuilder(builder))
    }
}

wrapped_clone_blob!("store", pub WrappedStore, SyncStore, defaults);
