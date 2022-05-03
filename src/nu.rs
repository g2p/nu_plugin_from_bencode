use crate::FromBencode;
use nu_errors::ShellError;
use nu_plugin::Plugin;
use nu_protocol::{Primitive, ReturnValue, Signature, UntaggedValue, Value};
use nu_source::Tag;

impl Plugin for FromBencode {
    fn config(&mut self) -> Result<Signature, ShellError> {
        Ok(Signature::build("from_bencode")
            .desc("Convert from bencode binary data into a table.")
            .filter())
    }

    fn filter(&mut self, input: Value) -> Result<Vec<ReturnValue>, ShellError> {
        match input {
            Value {
                value: UntaggedValue::Primitive(Primitive::Binary(b)),
                ..
            } => {
                self.buffer.extend_from_slice(&b);
            }
            Value { tag, .. } => {
                return Err(ShellError::labeled_error_with_secondary(
                    "Expected binary from pipeline",
                    "requires binary input",
                    Tag::unknown(),
                    "value originates from here",
                    tag,
                ));
            }
        }
        Ok(vec![])
    }

    fn end_filter(&mut self) -> Result<Vec<ReturnValue>, ShellError> {
        let mut buffer = Vec::new();
        std::mem::swap(&mut self.buffer, &mut buffer);
        crate::from_bencode::from_bytes_to_value(&buffer, Tag::unknown())
    }
}
