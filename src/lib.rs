#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(
    rust_2018_idioms,
    missing_docs,
    missing_debug_implementations,
    unused_lifetimes,
    unused_qualifications
)]

use nu_protocol::{Record, ShellError, Span, Value};

mod nu;

/// Converts bencode data to Nu structured values.
#[derive(Debug, Default)]
pub struct FromBencode;

fn convert_bencode_to_value(
    value: bt_bencode::Value,
    internal_span: Span,
) -> Result<Value, ShellError> {
    Ok(match value {
        bt_bencode::Value::Int(num) => match num {
            bt_bencode::value::Number::Signed(signed_num) => Value::int(signed_num, internal_span),
            bt_bencode::value::Number::Unsigned(unsigned_num) => i64::try_from(unsigned_num)
                .map(|val| Value::Int { val, internal_span })
                .map_err(|_| {
                    ShellError::UnsupportedInput(
                        "expected a compatible number".into(),
                        format!("{unsigned_num}"),
                        internal_span,
                        // TODO: The span is not correct, but there isn't a way to get the span of a value today.
                        internal_span,
                    )
                })?,
        },
        bt_bencode::Value::ByteStr(byte_str) => match String::from_utf8(byte_str.into_vec()) {
            Ok(s) => Value::string(s, internal_span),
            Err(err) => Value::binary(err.into_bytes(), internal_span),
        },
        bt_bencode::Value::List(list) => Value::list(
            list.into_iter()
                .map(|val| convert_bencode_to_value(val, internal_span))
                .collect::<Result<Vec<_>, ShellError>>()?,
            internal_span,
        ),
        bt_bencode::Value::Dict(dict) => {
            let mut record = Record::new();
            for (key, value) in dict {
                let key = String::from_utf8(key.into_vec()).map_err(|e| {
                    ShellError::UnsupportedInput(
                        format!("Unexpected bencode data {:?}:{:?}", e.into_bytes(), value),
                        "key is not a UTF-8 string".into(),
                        internal_span,
                        // TODO: The span is not correct, but there isn't a way to get the span of a value today.
                        internal_span,
                    )
                })?;
                let value = convert_bencode_to_value(value, internal_span)?;
                record.push(key, value);
            }

            Value::record(record, internal_span)
        }
    })
}

/// Converts a byte slice into a [`Value`].
///
/// # Errors
///
/// Returns an error if the input is not valid bencode data.
pub fn from_bytes_to_value(input: &[u8], head: Span) -> Result<Value, ShellError> {
    let value = bt_bencode::from_slice(input).map_err(|_e| ShellError::CantConvert {
        to_type: "bencode data".into(),
        from_type: "binary".into(),
        span: head,
        help: None,
    })?;
    convert_bencode_to_value(value, head)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_decode() -> Result<(), bt_bencode::Error> {
        let bencode_bytes = bt_bencode::to_vec(&bt_bencode::Value::from("hello world"))?;
        assert_eq!(bencode_bytes.len(), 14, "{bencode_bytes:?}");

        let internal_span = Span::new(0, bencode_bytes.len());
        let nu_value = from_bytes_to_value(&bencode_bytes, internal_span).unwrap();
        let expected = Value::String {
            val: "hello world".to_string(),
            internal_span,
        };
        assert_eq!(nu_value, expected);

        Ok(())
    }
}
