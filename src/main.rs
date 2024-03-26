use std::net::UdpSocket;

mod packet;
use packet::{PacketHeader, LapData, PacketType};
use byteorder::{ByteOrder, LittleEndian};

fn main() {
    println!("Running");
    let socket = UdpSocket::bind("192.168.1.26:20777").unwrap();

    // largest payload is 1464 bytes + 28 bytes for header = 1492
    let mut buf = [0u8; 1492];

    loop {
        // receive a single datagram message on the socket
        let (amt, src) = socket.recv_from(&mut buf).unwrap();

        // parse the packet
        parse_packet(&buf);
    }
}

// gets the packet type from the packet header
fn get_packet_type(buf: &[u8]) -> PacketType {
    let header = PacketHeader {
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
    };

    let packet_type = match header.m_packet_id {
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
    };

    packet_type
}

// parses the packet based on the packet type
// at least that what it will do lol
fn parse_packet(buf: &[u8]) {
  let header = get_packet_type(buf);
  match header {
    PacketType::LapData => {
      let lap_data = parse_lap_data(buf);
      let last_lap_time = lap_data.m_last_lap_time_in_ms;
      let current_lap_time = lap_data.m_current_lap_time_in_ms;
      let sector1_time = lap_data.m_sector1_time_in_ms;
      let sector2_time = lap_data.m_sector2_time_in_ms;
      let lap_distance = lap_data.m_lap_distance;
      let total_distance = lap_data.m_total_distance;

      println!("Last Lap Time: {}ms", last_lap_time);
      println!("Current Lap Time: {}ms", current_lap_time);
      println!("Sector 1 Time: {}ms", sector1_time);
      println!("Sector 2 Time: {}ms", sector2_time);
      println!("Lap Distance: {}", lap_distance);
      println!("Total Distance: {}", total_distance);
    },
    _ => {}
  }
}

// parses the lap data from the packet
fn parse_lap_data(buf: &[u8]) -> LapData {
    let lap_data = LapData {
        m_last_lap_time_in_ms: LittleEndian::read_u32(&buf[24..28]),
        m_current_lap_time_in_ms: LittleEndian::read_u32(&buf[28..32]),
        m_sector1_time_in_ms: LittleEndian::read_u16(&buf[32..34]),
        m_sector2_time_in_ms: LittleEndian::read_u16(&buf[34..36]),
        m_lap_distance: LittleEndian::read_f32(&buf[36..40]),
        m_total_distance: LittleEndian::read_f32(&buf[40..44]),
        m_safety_car_delta: LittleEndian::read_f32(&buf[44..48]),
        m_car_position: buf[48],
        m_current_lap_num: buf[49],
        m_pit_status: buf[50],
        m_num_pit_stops: buf[51],
        m_sector: buf[52],
        m_current_lap_invalid: buf[53],
        m_penalties: buf[54],
        m_warnings: buf[55],
        m_num_unserved_drive_through_pens: buf[56],
        m_num_unserved_stop_go_pens: buf[57],
        m_grid_position: buf[58],
        m_driver_status: buf[59],
        m_result_status: buf[60],
        m_pit_lane_timer_active: buf[61],
        m_pit_lane_time_in_lane_in_ms: LittleEndian::read_u16(&buf[62..64]),
        m_pit_stop_timer_in_ms: LittleEndian::read_u16(&buf[64..66]),
        m_pit_stop_should_serve_pen: buf[66]
    };

    lap_data
}


