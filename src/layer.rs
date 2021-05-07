use crate::store::*;
use std::io::{self, Write};
use std::iter::Peekable;
use swipl::prelude::*;
use terminus_store::layer::{IdTriple, ObjectType};
use terminus_store::storage::{name_to_string, string_to_name};
use terminus_store::store::sync::*;
use terminus_store::Layer;

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

    pub semidet fn node_and_value_count(_context, layer_term, count_term) {
        let layer: WrappedLayer = layer_term.get()?;
        let count = layer.node_and_value_count() as u64;

        count_term.unify(count)
    }

    pub semidet fn predicate_count(_context, layer_term, count_term) {
        let layer: WrappedLayer = layer_term.get()?;
        let count = layer.predicate_count() as u64;

        count_term.unify(count)
    }

    pub semidet fn subject_to_id(_context, layer_term, subject_term, id_term) {
        let layer: WrappedLayer = layer_term.get()?;
        let subject: PrologText = subject_term.get()?;

        match layer.subject_id(&subject) {
            Some(id) => id_term.unify(id),
            None => Err(PrologError::Failure)
        }
    }

    pub semidet fn id_to_subject(_context, layer_term, id_term, subject_term) {
        let layer: WrappedLayer = layer_term.get()?;
        let id: u64 = id_term.get()?;

        match layer.id_subject(id) {
            Some(subject) => subject_term.unify(subject),
            None => Err(PrologError::Failure)
        }
    }

    pub semidet fn predicate_to_id(_context, layer_term, predicate_term, id_term) {
        let layer: WrappedLayer = layer_term.get()?;
        let predicate: PrologText = predicate_term.get()?;

        match layer.predicate_id(&predicate) {
            Some(id) => id_term.unify(id),
            None => Err(PrologError::Failure)
        }
    }

    pub semidet fn id_to_predicate(_context, layer_term, id_term, predicate_term) {
        let layer: WrappedLayer = layer_term.get()?;
        let id: u64 = id_term.get()?;

        match layer.id_predicate(id) {
            Some(predicate) => predicate_term.unify(predicate),
            None => Err(PrologError::Failure)
        }
    }

    pub semidet fn object_to_id(context, layer_term, object_term, id_term) {
        let layer: WrappedLayer = layer_term.get()?;

        let inner = context.new_term_ref();
        let id: Option<u64>;
        if attempt(object_term.unify(term!{context: node(#&inner)}?))? {
            let object: PrologText = inner.get()?;
            id = layer.object_node_id(&object);
        }
        else if attempt(object_term.unify(term!{context: value(#&inner)}?))? {
            let object: PrologText = inner.get()?;
            id = layer.object_value_id(&object);
        }
        else {
            return context.raise_exception(&term!{context: error(domain_error(oneof([node(), value()]), #object_term), _)}?);
        }


        match id {
            Some(id) => id_term.unify(id),
            None => Err(PrologError::Failure)
        }
    }

    pub semidet fn id_to_object(_context, layer_term, id_term, object_term) {
        let layer: WrappedLayer = layer_term.get()?;
        let id: u64 = id_term.get()?;

        match layer.id_object(id) {
            Some(ObjectType::Node(object)) => {
                object_term.unify(Functor::new("node", 1))?;
                object_term.unify_arg(1, &object)
            }
            Some(ObjectType::Value(object)) => {
                object_term.unify(Functor::new("value", 1))?;
                object_term.unify_arg(1, &object)
            }
            None => Err(PrologError::Failure)
        }
    }

    pub semidet fn parent(context, layer_term, parent_term) {
        let layer: WrappedLayer = layer_term.get()?;
        match context.try_or_die(layer.parent())? {
            Some(p) => parent_term.unify(WrappedLayer(p)),
            None => Err(PrologError::Failure)
        }
    }

    pub semidet fn squash(context, layer_term, squashed_layer_term) {
        let layer: WrappedLayer = layer_term.get()?;
        let squashed = context.try_or_die(layer.squash())?;
        squashed_layer_term.unify(&WrappedLayer(squashed))
    }

    pub semidet fn rollup(context, layer_term) {
        let layer: WrappedLayer = layer_term.get()?;
        context.try_or_die(layer.rollup())
    }

    pub semidet fn rollup_upto(context, layer_term, upto_term) {
        let layer: WrappedLayer = layer_term.get()?;
        let upto: WrappedLayer = upto_term.get()?;
        context.try_or_die(layer.rollup_upto(&upto))
    }

    pub semidet fn imprecise_rollup_upto(context, layer_term, upto_term) {
        let layer: WrappedLayer = layer_term.get()?;
        let upto: WrappedLayer = upto_term.get()?;
        context.try_or_die(layer.imprecise_rollup_upto(&upto))
    }

    pub semidet fn layer_addition_count(context, layer_term, count_term) {
        let layer: WrappedLayer = layer_term.get()?;
        let count = context.try_or_die(layer.triple_layer_addition_count())? as u64;

        count_term.unify(count)
    }

    pub semidet fn layer_removal_count(context, layer_term, count_term) {
        let layer: WrappedLayer = layer_term.get()?;
        let count = context.try_or_die(layer.triple_layer_removal_count())? as u64;

        count_term.unify(count)
    }

    pub semidet fn layer_total_addition_count(_context, layer_term, count_term) {
        let layer: WrappedLayer = layer_term.get()?;
        let count = layer.triple_addition_count() as u64;

        count_term.unify(count)
    }

    pub semidet fn layer_total_removal_count(_context, layer_term, count_term) {
        let layer: WrappedLayer = layer_term.get()?;
        let count = layer.triple_removal_count() as u64;

        count_term.unify(count)
    }

    pub semidet fn layer_total_triple_count(_context, layer_term, count_term) {
        let layer: WrappedLayer = layer_term.get()?;
        let count = layer.triple_count() as u64;

        count_term.unify(count)
    }
}

wrapped_clone_blob!("layer", pub WrappedLayer, SyncStoreLayer);

impl CloneBlobImpl for WrappedLayer {
    fn write(&self, stream: &mut PrologStream) -> io::Result<()> {
        write!(stream, "<layer {}>", name_to_string(self.name()))
    }
}
