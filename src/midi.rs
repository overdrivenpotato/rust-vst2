use std::mem::transmute;

pub enum VoiceMessage {
    NoteOff         = 0b10000000,
    NoteOn          = 0b10010000,
    Aftertouch      = 0b10100000,
    ControlChange   = 0b10110000,
    ProgramChange   = 0b11000000,
    PitchBendChange = 0b11100000
}
impl VoiceMessage {
    fn from_bytes(b : &[u8]) -> Option<VoiceMessage> {
        match b[0] {
            0b10000000 => Some(VoiceMessage::NoteOff),
            0b10010000 => Some(VoiceMessage::NoteOn),
            0b10100000 => Some(VoiceMessage::Aftertouch),
            0b10110000 => Some(VoiceMessage::ControlChange),
            0b11000000 => Some(VoiceMessage::ProgramChange),
            0b11100000 => Some(VoiceMessage::PitchBendChange),
            _ => None
        }
    }
}

pub enum ChannelMode {
    OmniMode(bool),
    MonoMode(u8),
    PolyMode
}

pub enum ChannelMessage {
    AllSoundOff,
    ResetAllControllers,
    LocalControl(bool),
    AllNotesOff(Option<ChannelMode>)
}
impl ChannelMessage {
    fn from_bytes(b : &[u8]) -> Result<Self, bool> {
        if b[0] ^ 0b11111000 != 0 { return Err(false) }
        match b[0] & 0b00000111 {
            0b000 => Ok(ChannelMessage::AllSoundOff),
            0b001 => Ok(ChannelMessage::ResetAllControllers),
            0b010 => Ok(ChannelMessage::LocalControl(b[1] == 127)),
            0b011 => Ok(ChannelMessage::AllNotesOff(None)),
            0b100 => Ok(ChannelMessage::AllNotesOff(Some(ChannelMode::OmniMode(false)))),
            0b110 => Ok(ChannelMessage::AllNotesOff(Some(ChannelMode::OmniMode(true)))),
            0b101 => Ok(ChannelMessage::AllNotesOff(Some(ChannelMode::MonoMode(b[1])))),
            0b111 => Ok(ChannelMessage::AllNotesOff(Some(ChannelMode::PolyMode))),
            _ => unreachable!()
        }
    }
}

pub enum QuarterFrameType {
    FrameLSBs      = 0b00000000,
    FrameMSB       = 0b00010000,
    SecondLSBs     = 0b00100000,
    SecondMSBs     = 0b00110000,
    MinuteLSBs     = 0b01000000,
    MinuteMSBs     = 0b01010000,
    HourLSBs       = 0b01100000,
    HourMSBAndRate = 0b01110000,
}
impl From<u8> for QuarterFrameType {
    fn from(i:u8) -> Self {
        unsafe {
            transmute(i & 0b01110000)
        }
    }
}

pub enum SysMessage {
    SystemExclusive(Vec<u8>),
    QuarterFrame(QuarterFrameType, u8),
    SongPosition(u16),
    SongSelect(u8),
    TuneRequest,
    EndOfExclusive
}
impl SysMessage {
    fn from_bytes(b : &[u8]) -> Result<Self, bool> {
        if b[0] & 0b11110000 == 0 { return Err(false) }
        match b[0] & 0b00001111 {
            // TODO: Write handler for sysex
            0b0000 => Err(true),
            0b0001 => Ok(SysMessage::QuarterFrame(QuarterFrameType::from(b[1]), b[1] & 0b00001111)),
            0b0010 => Ok(SysMessage::SongPosition((b[1] as u16) << 8 + b[2] as u16)),
            0b0011 => Ok(SysMessage::SongSelect(b[1])),
            0b0110 => Ok(SysMessage::TuneRequest),
            0b0111 => Ok(SysMessage::EndOfExclusive),
            _ => Err(true)
        }
    }
}

pub enum SysRTMessage {
    TimingClock   = 0b11111000,
    Start         = 0b11111001,
    Continue      = 0b11111010,
    Stop          = 0b11111100,
    ActiveSensing = 0b11111110,
    Reset         = 0b11111111
}
impl SysRTMessage {
    fn from_bytes(b : &[u8]) -> Option<SysRTMessage>{
        match b[0] {
            0b11111000 => Some(SysRTMessage::TimingClock),
            0b11111001 => Some(SysRTMessage::Start),
            0b11111010 => Some(SysRTMessage::Continue),
            0b11111100 => Some(SysRTMessage::Stop),
            0b11111110 => Some(SysRTMessage::ActiveSensing),
            0b11111111 => Some(SysRTMessage::Reset),
            _ => None
        }
    }
}

pub enum Data {
    Voice(VoiceMessage, u8),
    Channel(ChannelMessage),
    System(SysMessage),
    SystemRealTime(SysRTMessage)
}

impl Data {
    pub fn from_bytes(b : &[u8]) -> Option<Data> {
        if let Some(i) = VoiceMessage::from_bytes(b) {return Some(Data::Voice(i, b[1])) }

        match ChannelMessage::from_bytes(b) {
            Ok(i) => return Some(Data::Channel(i)),
            Err(true) => return None,
            _ => ()
        }

        match SysMessage::from_bytes(b) {
            Ok(i) => return Some(Data::System(i)),
            Err(true) => return None,
            _ => ()
        }

        if let Some(i) = SysRTMessage::from_bytes(b) {return Some(Data::SystemRealTime(i)) }

        None
    }
}
