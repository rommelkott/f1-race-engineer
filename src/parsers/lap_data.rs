use byteorder::{ByteOrder, LittleEndian};

use crate::packet::LapData;

pub fn parse_lap_data(buf: &[u8]) -> LapData {
    LapData {
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
        m_pit_stop_should_serve_pen: buf[66],
    }
}
