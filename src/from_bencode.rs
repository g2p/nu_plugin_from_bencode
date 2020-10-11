use nu_errors::ShellError;
use nu_protocol::{Primitive, ReturnSuccess, ReturnValue, TaggedDictBuilder, UntaggedValue, Value};
use nu_source::Tag;

#[derive(Debug)]
pub struct FromBencode {
    pub buffer: Vec<u8>,
}

impl Default for FromBencode {
    fn default() -> Self {
        Self { buffer: Vec::new() }
    }
}

fn convert_bencode_value_to_nu_value<T>(
    value: bt_bencode::Value,
    tag: T,
) -> Result<Value, ShellError>
where
    T: Into<Tag>,
{
    let tag = tag.into();

    Ok(match value {
        bt_bencode::Value::Int(num) => match num {
            bt_bencode::value::Number::Signed(signed_num) => {
                UntaggedValue::int(signed_num).into_value(&tag)
            }
            bt_bencode::value::Number::Unsigned(unsigned_num) => {
                UntaggedValue::int(unsigned_num).into_value(&tag)
            }
        },
        bt_bencode::Value::ByteStr(byte_str) => {
            let bytes = byte_str.into_vec();
            match String::from_utf8(bytes.clone()) {
                Ok(s) => UntaggedValue::Primitive(Primitive::String(s)).into_value(&tag),
                Err(_) => UntaggedValue::Primitive(Primitive::Binary(bytes)).into_value(&tag),
            }
        }
        bt_bencode::Value::List(list) => UntaggedValue::Table(
            list.into_iter()
                .map(|v| convert_bencode_value_to_nu_value(v, tag.clone()))
                .collect::<Result<Vec<_>, ShellError>>()?,
        )
        .into_value(&tag),
        bt_bencode::Value::Dict(dict) => {
            let mut builder = TaggedDictBuilder::new(tag.clone());

            for (k, v) in dict {
                let bytes = k.into_vec();
                let key = match String::from_utf8(bytes.clone()) {
                    Ok(s) => s,
                    Err(_) => {
                        return Err(ShellError::labeled_error(
                            "Could not parse as Bencode",
                            "input cannot be parsed as Bencode",
                            tag,
                        ))
                    }
                };

                let value = convert_bencode_value_to_nu_value(v, &tag)?;

                builder.insert_value(key, value);
            }

            builder.into_value()
        }
    })
}

pub fn from_bencode_bytes_to_value<T>(
    bytes: Vec<u8>,
    tag: T,
) -> Result<Vec<ReturnValue>, ShellError>
where
    T: Into<Tag>,
{
    let mut values: Vec<bt_bencode::Value> = Vec::new();

    let reader = std::rc::Rc::new(std::cell::RefCell::new(bt_bencode::read::SliceRead::new(
        &bytes[..],
    )));

    loop {
        let mut de = bt_bencode::Deserializer::new(crate::read::SliceReadView::new(reader.clone()));

        use bt_bencode::read::Read;
        use serde::de::Deserialize;

        match bt_bencode::Value::deserialize(&mut de) {
            Ok(value) => {
                values.push(value);
            }
            Err(e) => match e {
                bt_bencode::Error::EofWhileParsingValue
                    if reader.clone().borrow_mut().peek().is_none() =>
                {
                    break;
                }
                _ => {
                    return Err(ShellError::labeled_error(
                        "Could not parse as Bencode data",
                        "input cannot be parsed as Bencode",
                        tag.into(),
                    ));
                }
            },
        }
    }

    let tag = tag.into();
    Ok(values
        .into_iter()
        .map(|v| convert_bencode_value_to_nu_value(v, tag.clone()).map(ReturnSuccess::value))
        .collect::<Result<Vec<ReturnValue>, ShellError>>()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_multiple() -> Result<(), bt_bencode::Error> {
        let mut bencode_bytes = Vec::new();
        bencode_bytes.extend_from_slice(&bt_bencode::to_vec(&bt_bencode::Value::from(
            "hello world",
        ))?);
        bencode_bytes.extend_from_slice(&bt_bencode::to_vec(&bt_bencode::Value::from(128))?);

        assert_eq!(bencode_bytes.len(), 19, "{:?}", bencode_bytes);

        let nu_value = from_bencode_bytes_to_value(bencode_bytes, Tag::unknown());
        match nu_value {
            Ok(values) => {
                assert_eq!(values.len(), 2);
                let expected = vec![
                    Some(Value::from("hello world")),
                    Some(UntaggedValue::int(128).into_value(&Tag::unknown())),
                    None,
                ];
                for (actual, expected) in values.into_iter().zip(expected.into_iter()) {
                    assert_eq!(actual.unwrap().raw_value(), expected);
                }
            }
            Err(e) => panic!("{}", e),
        }

        Ok(())
    }
}
