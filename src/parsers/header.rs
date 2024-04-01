use byteorder::{ByteOrder, LittleEndian};

use crate::packet::{PacketHeader, PacketType};

pub fn parse_header(buf: &[u8]) -> PacketHeader {
  PacketHeader {
    m_packet_format: LittleEndian::read_u16(&buf[0..2]),
    m_game_major_version: buf[2],
    m_game_minor_version: buf[3],
    m_packet_version: buf[4],
    m_packet_id: buf[5],
    m_session_uid: LittleEndian::read_u64(&buf[6..14]),
    m_session_time: LittleEndian::read_f32(&buf[14..18]),
    m_frame_identifier: LittleEndian::read_u32(&buf[18..22]),
    m_player_car_index: buf[22],
    m_secondary_player_car_index: buf[23]
  }
}

pub fn get_packet_type(buf: &[u8]) -> PacketType {
  match buf[5] {
    0 => PacketType::Motion,
    1 => PacketType::Session,
    2 => PacketType::LapData,
    3 => PacketType::Event,
    4 => PacketType::Participants,
    5 => PacketType::CarSetups,
    6 => PacketType::CarTelemetry,
    7 => PacketType::CarStatus,
    8 => PacketType::FinalClassification,
    9 => PacketType::LobbyInfo,
    10 => PacketType::CarDamage,
    11 => PacketType::SessionHistory,
    _ => panic!("Unknown packet type")
  }
}

