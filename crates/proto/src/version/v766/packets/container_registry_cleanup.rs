use crate::version::v766::types::FullContainerName;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub struct ContainerRegistryCleanupPacket {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub removed_containers: Vec<FullContainerName>,
}