pub mod test_shared;

mod mav_frame_tests {
    // NOTE: No header
    pub const HEARTBEAT_V2: &[u8] = &[
        0xef, // seq 239
        0x01, // sys ID
        0x01, // comp ID
        0x00, 0x00, 0x00, // msg ID
        0x05, 0x00, 0x00, 0x00, 0x02, 0x03, 0x59, 0x03, 0x03, // payload
        16, 240, // checksum
    ];

    #[test]
    pub fn test_deser() {
        use mavlink::{common::MavMessage, MavFrame, MavlinkVersion};
        let frame = MavFrame::<MavMessage>::deser(MavlinkVersion::V2, HEARTBEAT_V2)
            .expect("failed to parse message");

        assert_eq!(frame.header, crate::test_shared::COMMON_MSG_HEADER);
        let heartbeat_msg = crate::test_shared::get_heartbeat_msg();

        let msg = match frame.msg {
            MavMessage::HEARTBEAT(msg) => msg,
            _ => panic!("Decoded wrong message type"),
        };
        assert_eq!(msg.custom_mode, heartbeat_msg.custom_mode);
        assert_eq!(msg.mavtype, heartbeat_msg.mavtype);
        assert_eq!(msg.autopilot, heartbeat_msg.autopilot);
        assert_eq!(msg.base_mode, heartbeat_msg.base_mode);
        assert_eq!(msg.system_status, heartbeat_msg.system_status);
        assert_eq!(msg.mavlink_version, heartbeat_msg.mavlink_version);
    }
}
