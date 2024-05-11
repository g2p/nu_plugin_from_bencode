use crate::{from_bytes_to_value, FromBencodePlugin};
use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Category, LabeledError, Signature, Type, Value};

pub struct FromBencodeCommand;

impl SimplePluginCommand for FromBencodeCommand {
    type Plugin = FromBencodePlugin;

    fn name(&self) -> &str {
        "from bencode"
    }

    fn usage(&self) -> &str {
        "Parse data as bencode and create table."
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_types(vec![(Type::Binary, Type::table())])
            .category(Category::Formats)
    }

    fn run(
        &self,
        _plugin: &FromBencodePlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        from_bencode(call, input)
    }
}

fn from_bencode(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    let head = call.head;

    let binary_input = input.as_binary()?;

    if binary_input.is_empty() {
        return Ok(Value::Nothing {
            internal_span: head,
        });
    }

    Ok(from_bytes_to_value(binary_input, head)?)
}
