use bedrock_core::VAR;
use proto_derive::ProtoCodec;

#[derive(Debug, Clone, ProtoCodec)]
pub struct PacketViolationWarningPacket {
    kind: VAR<u32>,
    severity: VAR<u32>,
    violating_packet_id: VAR<u32>,
    context: String,
}