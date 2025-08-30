pub mod common;
#[allow(dead_code)]
pub mod counter;
mod crdts;
pub mod datatype;
mod mutable;
mod transactional;

macro_rules! datatype_instrument {

    ($vis:vis fn $name:ident $($rest:tt)*) => {
        #[tracing::instrument(skip(self),
            fields(
                syncyam.dt=%self.datatype.attr.key,
                syncyam.duid=%self.datatype.attr.duid,
            )
        )]
        $vis fn $name $($rest)*
    };
}

pub(crate) use datatype_instrument;
