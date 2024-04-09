use mina_hasher::{Hashable, ROInput};
use nacho_data_structures::Field;

/// A wrapper around an array of `Field` elements to faciliate hashing.
#[derive(Clone, Debug)]
pub(crate) struct Message(pub(crate) Vec<Field>);

impl<'a> Hashable for Message {
    type D = ();

    fn domain_string(_: Self::D) -> Option<String> {
        Some("CodaSignature*******".to_string())
    }

    fn to_roinput(&self) -> mina_hasher::ROInput {
        let mut roinput = ROInput::new();

        for field in self.0.clone() {
            roinput = roinput.append_field(field)
        }

        roinput
    }
}
