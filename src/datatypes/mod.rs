pub mod common;
#[allow(dead_code)]
pub mod counter;
mod crdts;
pub mod datatype;
mod mutable;
mod rollback;
mod transactional;

macro_rules! datatype_instrument {
    ($(#[$attr:meta])* $vis:vis fn $name:ident $($rest:tt)*) => {
        $(#[$attr])*
        #[tracing::instrument(skip_all,
            fields(
                syncyam.dt=%self.datatype.attr.key,
                syncyam.duid=%self.datatype.attr.duid,
            )
        )]
        $vis fn $name $($rest)*
    };
}

pub(crate) use datatype_instrument;
