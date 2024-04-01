
#[repr(C, packed)]
#[derive(Debug)]
pub struct PacketHeader {
  pub m_packet_format: u16,
  pub m_game_major_version: u8,
  pub m_game_minor_version: u8,
  pub m_packet_version: u8,
  pub m_packet_id: u8,
  pub m_session_uid: u64,
  pub m_session_time: f32,
  pub m_frame_identifier: u32,
  pub m_player_car_index: u8,
  pub m_secondary_player_car_index: u8
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct LapData {
    pub m_last_lap_time_in_ms: u32,
    pub m_current_lap_time_in_ms: u32,
    pub m_sector1_time_in_ms: u16,
    pub m_sector2_time_in_ms: u16,
    pub m_lap_distance: f32,
    pub m_total_distance: f32,
    pub m_safety_car_delta: f32,
    pub m_car_position: u8,
    pub m_current_lap_num: u8,
    pub m_pit_status: u8,
    pub m_num_pit_stops: u8,
    pub m_sector: u8,
    pub m_current_lap_invalid: u8,
    pub m_penalties: u8,
    pub m_warnings: u8,
    pub m_num_unserved_drive_through_pens: u8,
    pub m_num_unserved_stop_go_pens: u8,
    pub m_grid_position: u8,
    pub m_driver_status: u8,
    pub m_result_status: u8,
    pub m_pit_lane_timer_active: u8,
    pub m_pit_lane_time_in_lane_in_ms: u16,
    pub m_pit_stop_timer_in_ms: u16,
    pub m_pit_stop_should_serve_pen: u8
}

#[derive(Debug)]
pub enum PacketType {
  Motion,
  Session,
  LapData,
  Event,
  Participants,
  CarSetups,
  CarTelemetry,
  CarStatus,
  FinalClassification,
  LobbyInfo,
  CarDamage,
  SessionHistory,
}

#[derive(Debug)]
pub struct Context {
  pub lap_data: LapData
}

impl Context {
  pub fn new() -> Self {
    Context {
      lap_data: LapData {
        m_last_lap_time_in_ms: 0,
        m_current_lap_time_in_ms: 0,
        m_sector1_time_in_ms: 0,
        m_sector2_time_in_ms: 0,
        m_lap_distance: 0.0,
        m_total_distance: 0.0,
        m_safety_car_delta: 0.0,
        m_car_position: 0,
        m_current_lap_num: 0,
        m_pit_status: 0,
        m_num_pit_stops: 0,
        m_sector: 0,
        m_current_lap_invalid: 0,
        m_penalties: 0,
        m_warnings: 0,
        m_num_unserved_drive_through_pens: 0,
        m_num_unserved_stop_go_pens: 0,
        m_grid_position: 0,
        m_driver_status: 0,
        m_result_status: 0,
        m_pit_lane_timer_active: 0,
        m_pit_lane_time_in_lane_in_ms: 0,
        m_pit_stop_timer_in_ms: 0,
        m_pit_stop_should_serve_pen: 0
      }
    }
  }
}