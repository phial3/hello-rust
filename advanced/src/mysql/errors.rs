#![allow(dead_code)]

pub type MySQLResult<T> = std::result::Result<T, MySQLError>;

#[derive(Debug)]
pub enum MySQLError {
    MismatchPacketSequence,
    PacketZeroPayload,
    OkPacketWrongSize,
    OkPacketILL,
    ErrPacketWrongSize,
    ErrPacketILL,
    ErUnknownCmd,
    IO(std::io::Error),
}

impl From<std::io::Error> for MySQLError {
    fn from(e: std::io::Error) -> Self {
        MySQLError::IO(e)
    }
}

impl std::error::Error for MySQLError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MySQLError::MismatchPacketSequence => None,
            MySQLError::PacketZeroPayload => None,
            MySQLError::OkPacketILL => None,
            MySQLError::ErUnknownCmd => None,
            MySQLError::OkPacketWrongSize => None,
            MySQLError::ErrPacketWrongSize => None,
            MySQLError::ErrPacketILL => None,
            MySQLError::IO(e) => e.source(),
        }
    }
}

impl std::fmt::Display for MySQLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MySQLError::MismatchPacketSequence => write!(f, "MysqlError::MismatchPacketSequence!"),
            MySQLError::PacketZeroPayload => write!(f, "MysqlError::PacketZeroPayload!"),
            MySQLError::OkPacketWrongSize => write!(f, "MysqlError::OkPacketWrongSize!"),
            MySQLError::OkPacketILL => write!(f, "MysqlError::OkPacketILL!"),
            MySQLError::ErUnknownCmd => write!(f, "MysqlError::ErUnknownCmd!"),
            MySQLError::ErrPacketWrongSize => write!(f, "MysqlError::ErrPacketWrongSize!"),
            MySQLError::ErrPacketILL => write!(f, "MysqlError::ErrPacketILL!"),
            MySQLError::IO(e) => e.fmt(f),
        }
    }
}
