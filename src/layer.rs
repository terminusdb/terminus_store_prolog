use swipl::prelude::*;
use terminus_store::storage::{name_to_string, string_to_name};
use terminus_store::store::sync::*;
use terminus_store::Layer;
use std::io::{self, Write};
use crate::store::*;
use crate::builder::*;

predicates! {
    pub semidet fn store_id_layer(context, store_term, id_term, layer_term) {
        let store: WrappedStore = store_term.get()?;
        if id_term.is_var() {
            // get id from layer, which has to be there
            let layer: WrappedLayer = layer_term.get()?;
            let id = layer.name();
            let id_string = name_to_string(id);
            id_term.unify(&id_string)
        }
        else {
            // load layer by id
            let id_string: String = id_term.get()?;
            let id = context.try_or_die(string_to_name(&id_string))?;
            match context.try_or_die(store.get_layer_from_id(id))? {
                None => Err(PrologError::Failure),
                Some(layer) => layer_term.unify(&WrappedLayer(layer)),
            }
        }
    }

    pub semidet fn open_write(context, layer_term, builder_term) {
        let layer: WrappedLayer = layer_term.get()?;
        let builder = context.try_or_die(layer.open_write())?;
        builder_term.unify(WrappedBuilder(builder))
    }
}

wrapped_clone_blob!("layer", pub WrappedLayer, SyncStoreLayer);

impl CloneBlobImpl for WrappedLayer {
    fn write(&self, stream: &mut PrologStream) -> io::Result<()> {
        write!(stream, "<layer {}>", name_to_string(self.name()))
    }
}
