use swipl::prelude::*;
use terminus_store::store::sync::*;
use std::sync::Arc;
use std::io::{self, Write};
use crate::store::*;
use crate::layer::*;

predicates! {
    pub semidet fn create_named_graph(context, store_term, graph_name_term, graph_term) {
        let store: WrappedStore = store_term.get()?;
        let graph_name: String = graph_name_term.get()?;

        let graph = context.try_or_die(store.create(&graph_name))?;
        graph_term.unify(&WrappedNamedGraph(Arc::new(graph)))
    }

    pub semidet fn open_named_graph(context, store_term, graph_name_term, graph_term) {
        let store: WrappedStore = store_term.get()?;
        let graph_name: String = graph_name_term.get()?;

        match context.try_or_die(store.open(&graph_name))? {
            None => Err(PrologError::Failure),
            Some(graph) => graph_term.unify(&WrappedNamedGraph(Arc::new(graph))),
        }
    }

    #[name("head")]
    pub semidet fn head2(context, graph_term, layer_term) {
        let graph: WrappedNamedGraph = graph_term.get()?;
        match context.try_or_die(graph.head())? {
            None => Err(PrologError::Failure),
            Some(layer) => layer_term.unify(&WrappedLayer(Arc::new(layer))),
        }
    }

    #[name("head")]
    pub semidet fn head3(context, graph_term, layer_term, version_term) {
        let graph: WrappedNamedGraph = graph_term.get()?;
        let (layer_opt, version) = context.try_or_die(graph.head_version())?;
        version_term.unify(version)?;

        if let Some(layer) = layer_opt {
                layer_term.unify(&WrappedLayer(Arc::new(layer)))?;
        }

        Ok(())
    }

    pub semidet fn nb_set_head(context, graph_term, layer_term) {
        let graph: WrappedNamedGraph = graph_term.get()?;
        let layer: WrappedLayer = layer_term.get()?;

        into_prolog_result(context.try_or_die(graph.set_head(&layer))?)
    }
}

wrapped_arc_blob!(pub "named_graph", WrappedNamedGraph, SyncNamedGraph);

impl WrappedArcBlobImpl for WrappedNamedGraph {
    fn write(this: &SyncNamedGraph, stream: &mut PrologStream) -> io::Result<()> {
        write!(stream, "<named_graph {}>", this.name())
    }
}
