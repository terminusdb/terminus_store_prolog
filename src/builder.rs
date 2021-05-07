use crate::layer::*;
use std::io::{self, Write};
use swipl::prelude::*;
use terminus_store::layer::*;
use terminus_store::storage::name_to_string;
use terminus_store::store::sync::*;

predicates! {
    pub semidet fn nb_add_id_triple(context, builder_term, subject_id_term, predicate_id_term, object_id_term) {
        let builder: WrappedBuilder = builder_term.get()?;
        let subject_id = subject_id_term.get()?;
        let predicate_id = predicate_id_term.get()?;
        let object_id = object_id_term.get()?;

        context.try_or_die(builder.add_id_triple(IdTriple::new(subject_id, predicate_id, object_id)))
    }

    pub semidet fn nb_add_string_triple(context, builder_term, subject_term, predicate_term, object_term) {
        let builder: WrappedBuilder = builder_term.get()?;
        let subject: PrologText = subject_term.get()?;
        let predicate: PrologText = predicate_term.get()?;

        let inner = context.new_term_ref();
        if attempt(object_term.unify(term!{context: node(#&inner)}?))? {
            let object: PrologText = inner.get()?;
            context.try_or_die(builder.add_string_triple(StringTriple::new_node(&subject, &predicate, &object)))
        }
        else if attempt(object_term.unify(term!{context: value(#&inner)}?))? {
            let object: PrologText = inner.get()?;
            context.try_or_die(builder.add_string_triple(StringTriple::new_value(&subject, &predicate, &object)))
        }
        else {
            context.raise_exception(&term!{context: error(domain_error(oneof([node(), value()]), #object_term), _)}?)
        }
    }

    pub semidet fn nb_remove_id_triple(context, builder_term, subject_id_term, predicate_id_term, object_id_term) {
        let builder: WrappedBuilder = builder_term.get()?;
        let subject_id = subject_id_term.get()?;
        let predicate_id = predicate_id_term.get()?;
        let object_id = object_id_term.get()?;

        context.try_or_die(builder.remove_id_triple(IdTriple::new(subject_id, predicate_id, object_id)))
    }

    pub semidet fn nb_remove_string_triple(context, builder_term, subject_term, predicate_term, object_term) {
        let builder: WrappedBuilder = builder_term.get()?;
        let subject: PrologText = subject_term.get()?;
        let predicate: PrologText = predicate_term.get()?;

        let inner = context.new_term_ref();
        if attempt(object_term.unify(term!{context: node(#&inner)}?))? {
            let object: PrologText = inner.get()?;
            context.try_or_die(builder.remove_string_triple(StringTriple::new_node(&subject, &predicate, &object)))
        }
        else if attempt(object_term.unify(term!{context: value(#&inner)}?))? {
            let object: PrologText = inner.get()?;
            context.try_or_die(builder.remove_string_triple(StringTriple::new_value(&subject, &predicate, &object)))
        }
        else {
            context.raise_exception(&term!{context: error(domain_error(oneof([node(), value()]), #object_term), _)}?)
        }
    }

    pub semidet fn builder_committed(_context, builder_term) {
        let builder: WrappedBuilder = builder_term.get()?;

        into_prolog_result(builder.committed())
    }

    pub semidet fn nb_commit(context, builder_term, layer_term) {
        let builder: WrappedBuilder = builder_term.get()?;
        let layer = context.try_or_die(builder.commit())?;
        layer_term.unify(WrappedLayer(layer))
    }

    pub semidet fn apply_delta(context, builder_term, layer_term) {
        let builder: WrappedBuilder = builder_term.get()?;
        let layer: WrappedLayer = layer_term.get()?;

        context.try_or_die(builder.apply_delta(&layer))?;

        Ok(())
    }

    pub semidet fn apply_diff(context, builder_term, layer_term) {
        let builder: WrappedBuilder = builder_term.get()?;
        let layer: WrappedLayer = layer_term.get()?;

        context.try_or_die(builder.apply_diff(&layer))?;

        Ok(())
    }
}

wrapped_clone_blob!("builder", pub WrappedBuilder, SyncStoreLayerBuilder);

impl CloneBlobImpl for WrappedBuilder {
    fn write(&self, stream: &mut PrologStream) -> io::Result<()> {
        write!(stream, "<builder {}>", name_to_string(self.name()))
    }
}
