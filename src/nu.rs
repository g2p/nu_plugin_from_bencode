use crate::{from_bytes_to_value, FromBencode};
use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Category, PluginSignature, Value};

const FROM_BENCODE_COMMAND: &str = "from bencode";

impl Plugin for FromBencode {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build(FROM_BENCODE_COMMAND)
            .usage("Parse data as bencode and create table.")
            .category(Category::Formats)]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        match name {
            FROM_BENCODE_COMMAND => {
                from_bencode(call, input)
            },
            _ => Err(LabeledError {
                label: "Plugin call with wrong name signature".into(),
                msg: "the signature used to call the plugin does not match any name in the plugin signature vector".into(),
                span: Some(call.head),
            }),
        }
    }
}

fn from_bencode(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    let head = call.head;

    let binary_input = input.as_binary()?;

    if binary_input.is_empty() {
        return Ok(Value::Nothing { span: head });
    }

    Ok(from_bytes_to_value(binary_input, head)?)
}
