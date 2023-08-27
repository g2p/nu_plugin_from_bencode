use nu_plugin::JsonSerializer;
use nu_plugin_from_bencode::FromBencode;

fn main() {
    nu_plugin::serve_plugin(&mut FromBencode, JsonSerializer {});
}
