use bytesize::ByteSize;
use std::time;

pub const CLEAR_SCREEN: &str = "\x1B[2J\x1B[1;1H";
pub const CLEAR_PREV_LINE: &str = "\x1b[0F\x1b[2K";

#[repr(u8)]
pub enum Command {
    Upload = 0x02,   // ASCII control characters: "Start of Text"
    Download = 0x03, // ASCII control characters: "End of Text"
    Finish = 0x04,   // ASCII control characters: "End of Transmission"
}

pub fn fmt_speed(s: usize, t: time::Duration) -> String {
    fmt_bytes((s as u128 * 1_000_000_000 / t.as_nanos()) as usize) + "/s"
}

pub fn fmt_bytes(s: usize) -> String {
    ByteSize(s as u64).to_string_as(true)
}

pub fn fmt_duration(t: time::Duration) -> String {
    format!("{:.1}s", t.as_secs_f32())
}
