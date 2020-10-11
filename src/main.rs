use nu_plugin::serve_plugin;
use nu_plugin_from_bencode::FromBencode;

fn main() {
    serve_plugin(&mut FromBencode::default());
}
