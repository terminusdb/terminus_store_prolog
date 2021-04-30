use swipl::prelude::*;
use terminus_store::store::sync::*;
use terminus_store::storage::name_to_string;
use std::io::{self, Write};
use crate::layer::*;

predicates! {
    pub semidet fn nb_commit(context, builder_term, layer_term) {
        let builder: WrappedBuilder = builder_term.get()?;
        let layer = context.try_or_die(builder.commit())?;
        layer_term.unify(WrappedLayer(layer))
    }
}

wrapped_clone_blob!("builder", pub WrappedBuilder, SyncStoreLayerBuilder);

impl CloneBlobImpl for WrappedBuilder {
    fn write(&self, stream: &mut PrologStream) -> io::Result<()> {
        write!(stream, "<builder {}>", name_to_string(self.name()))
    }
}
