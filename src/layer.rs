use swipl::prelude::*;
use terminus_store::storage::{name_to_string, string_to_name};
use terminus_store::store::sync::*;
use terminus_store::Layer;
use std::sync::Arc;
use std::io::{self, Write};
use crate::store::*;
use crate::builder::*;

predicates! {
    semidet fn store_id_layer(context, store_term, id_term, layer_term) {
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
            match string_to_name(&id_string) {
                Ok(id) => {
                    match store.get_layer_from_id(id) {
                        Ok(None) => Err(PrologError::Failure),
                        Ok(Some(layer)) => layer_term.unify(&WrappedLayer(Arc::new(layer))),
                        Err(e) => {
                            let msg = format!("{}", e);
                            let error_term = term!{context: error(rust_error(#msg), _)};

                            context.raise_exception(&error_term)
                        }
                    }
                },
                Err(e) => {
                    let msg = format!("{}", e);
                    let error_term = term!{context: error(rust_error(#msg), _)};

                    context.raise_exception(&error_term)
                }
            }
        }
    }

    semidet fn open_write(context, layer_term, builder_term) {
        let layer: WrappedLayer = layer_term.get()?;
        match layer.open_write() {
            Ok(builder) => {
                builder_term.unify(WrappedBuilder(Arc::new(builder)))
            }
            Err(e) => {
                let msg = format!("{}", e);
                let error_term = term!{context: error(rust_error(#msg), _)};

                context.raise_exception(&error_term)
            }
        }
    }
}

wrapped_arc_blob!(pub "layer", WrappedLayer, SyncStoreLayer);

impl WrappedArcBlobImpl for WrappedLayer {
    fn write(this: &SyncStoreLayer, stream: &mut PrologStream) -> io::Result<()> {
        write!(stream, "<layer {}>", name_to_string(this.name()))
    }
}
