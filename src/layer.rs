use crate::store::*;
use std::io::{self, Write};
use std::iter::Peekable;
use swipl::prelude::*;
use terminus_store::storage::{name_to_string, string_to_name};
use terminus_store::store::sync::*;
use terminus_store::Layer;
use terminus_store::layer::IdTriple;

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
            let id_string: PrologText = id_term.get()?;
            let id = context.try_or_die(string_to_name(&id_string))?;
            match context.try_or_die(store.get_layer_from_id(id))? {
                None => Err(PrologError::Failure),
                Some(layer) => layer_term.unify(&WrappedLayer(layer)),
            }
        }
    }

    pub nondet fn id_triple<Peekable<Box<dyn Iterator<Item=IdTriple>+Send>>>(context, layer_term, subject_id_term, predicate_id_term, object_id_term) {
        setup => {
            let layer: WrappedLayer = layer_term.get()?;

            let iter: Box<dyn Iterator<Item=IdTriple>+Send>;
            if let Some(subject_id) = attempt_opt(subject_id_term.get::<u64>())? {
                if let Some(predicate_id) = attempt_opt(predicate_id_term.get::<u64>())? {
                    if let Some(object_id) = attempt_opt(object_id_term.get::<u64>())? {
                        // everything is known
                        if layer.triple_exists(subject_id, predicate_id, object_id) {
                            return Ok(None);
                        }
                        else {
                            return Err(PrologError::Failure)
                        }
                    }
                    else {
                        // subject and predicate are known, object is not
                        iter = layer.triples_sp(subject_id, predicate_id);
                    }
                }
                else {
                    // subject is known, predicate is not. object may or may not be bound already.
                    if let Some(object_id) = attempt_opt(object_id_term.get::<u64>())? {
                        // object is known so predicate is the only unknown
                        iter = Box::new(layer.triples_s(subject_id)
                                        .filter(move |t| t.object == object_id));
                    }
                    else {
                        // both predicate and object are unknown
                        iter = layer.triples_s(subject_id);
                    }
                }
            }
            else if let Some(object_id) = attempt_opt(object_id_term.get::<u64>())? {
                // subject is unknown
                if let Some(predicate_id) = attempt_opt(predicate_id_term.get::<u64>())? {
                    // predicate is known
                    iter = Box::new(layer.triples_o(object_id)
                                    .filter(move |t| t.predicate == predicate_id));
                }
                else {
                    // predicate is unknown, only object is known
                    iter = layer.triples_o(object_id)
                }
            }
            else if let Some(predicate_id) = attempt_opt(predicate_id_term.get::<u64>())? {
                // only predicate is known
                iter = layer.triples_p(predicate_id);
            }
            else {
                // nothing is known so return everything
                iter = layer.triples();
            }

            // lets make it peekable
            let iter = iter.peekable();

            Ok(Some(iter))
        },
        call(iter) => {
            if let Some(triple) = iter.next() {
                subject_id_term.unify(triple.subject)?;
                predicate_id_term.unify(triple.predicate)?;
                object_id_term.unify(triple.object)?;

                Ok(iter.peek().is_some())
            }
            else {
                return Err(PrologError::Failure);
            }
        }
    }
}

wrapped_clone_blob!("layer", pub WrappedLayer, SyncStoreLayer);

impl CloneBlobImpl for WrappedLayer {
    fn write(&self, stream: &mut PrologStream) -> io::Result<()> {
        write!(stream, "<layer {}>", name_to_string(self.name()))
    }
}
