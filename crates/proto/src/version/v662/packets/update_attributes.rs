use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::{AttributeModifierOperation, AttributeOperands};
use crate::version::v662::types::ActorRuntimeID;

#[derive(ProtoCodec)]
struct AttributeModifier {
    pub id: String,
    pub name: String,
    #[endianness(le)]
    pub amount: f32,
    pub operation: AttributeModifierOperation,
    pub operand: AttributeOperands,
    pub is_serializable: bool,
}

#[derive(ProtoCodec)]
struct AttributeData {
    #[endianness(le)]
    pub min_value: f32,
    #[endianness(le)]
    pub max_value: f32,
    #[endianness(le)]
    pub current_value: f32,
    #[endianness(le)]
    pub default_value: f32,
    pub attribute_name: String,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub attribute_modifiers: Vec<AttributeModifier>,
}

#[gamepacket(id = 29)]
#[derive(ProtoCodec)]
pub struct UpdateAttributesPacket {
    pub target_runtime_id: ActorRuntimeID,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub attribute_list: Vec<AttributeData>,
    #[endianness(var)]
    pub ticks_since_sim_started: u64,
}