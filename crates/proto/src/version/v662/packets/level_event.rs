use crate::version::v662::enums::LevelEvent;
use vek::Vec3;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 25)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct LevelEventPacket {
    pub event_id: LevelEvent,
    #[endianness(le)]
    pub position: Vec3<f32>,
    #[endianness(var)]
    pub data: i32,
}