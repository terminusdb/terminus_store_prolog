use swipl::prelude::*;
use terminus_store::store::sync::*;
use terminus_store::storage::name_to_string;
use std::io::{self, Write};
use std::sync::Arc;
use crate::layer::*;

predicates! {
    semidet fn nb_commit(context, builder_term, layer_term) {
        let builder: WrappedBuilder = builder_term.get()?;
        let layer = context.except(builder.commit())?;
        layer_term.unify(WrappedLayer(Arc::new(layer)))
    }
}

wrapped_arc_blob!(pub "builder", WrappedBuilder, SyncStoreLayerBuilder);

impl WrappedArcBlobImpl for WrappedBuilder {
    fn write(this: &SyncStoreLayerBuilder, stream: &mut PrologStream) -> io::Result<()> {
        write!(stream, "<builder {}>", name_to_string(this.name()))
    }
}
