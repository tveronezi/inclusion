use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../inclusion-ui/inclusion-ui/build/"]
pub struct Assets;
