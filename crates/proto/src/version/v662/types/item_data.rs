use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub struct ItemData {
    pub name: String,
    #[endianness(le)]
    pub id: i16,
    pub is_component_based: bool,
}
