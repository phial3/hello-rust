#![allow(dead_code)]
use super::constants::MAX_PAYLOAD_LEN;
use crate::mysql::errors::{MySQLError, MySQLResult};
use byteorder::{LittleEndian as LE, WriteBytesExt};
use std::io::Cursor;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct PacketIO {
    sequence: u8,
    stream: TcpStream,
}

impl PacketIO {
    pub fn new(s: TcpStream) -> PacketIO {
        PacketIO {
            sequence: 0u8,
            stream: s,
        }
    }
    pub fn quit(&self) -> MySQLResult<()> {
        //self.stream.shutdown();
        Ok(())
    }

    pub fn reset_seq(&mut self) {
        self.sequence = 0;
    }

    pub async fn read_packet(&mut self) -> MySQLResult<Vec<u8>> {
        let mut prev_data: Vec<u8> = Vec::new();
        loop {
            let mut header = [0u8; 4];
            self.stream.read_exact(&mut header).await?;
            if header[3] != self.sequence {
                return Err(MySQLError::MismatchPacketSequence);
            }

            self.sequence += 1;

            //let payload_len = (header[0] as u32) |  ((header[1] as u32) << 8) | ((header[2]as u32) << 16) ;
            let mut rdr = Cursor::new(&header[..3]);
            let payload_len =
                byteorder::ReadBytesExt::read_u24::<LE>(&mut rdr).unwrap_or(0) as usize;
            // packets with length 0 terminate a previous packet which is a
            // multiple of (2^24)-1 bytes long
            if payload_len == 0 {
                // there was no previous packet
                if prev_data.is_empty() {
                    return Err(MySQLError::PacketZeroPayload);
                }
                return Ok(prev_data);
            }

            let mut buf = vec![0; payload_len];
            self.stream.read_exact(&mut buf).await?;
            if payload_len < MAX_PAYLOAD_LEN {
                if prev_data.is_empty() {
                    return Ok(buf);
                }
                prev_data.append(&mut buf);
                return Ok(prev_data);
            }
            prev_data.append(&mut buf);
        } //end of loop
    }

    //attention: do not included header in data
    pub async fn write_packet(&mut self, data: &mut [u8]) -> MySQLResult<()> {
        let mut data_len = data.len();
        let mut header = vec![0u8; 4];
        header.extend_from_slice(data);
        let mut bufp = header.as_mut_slice();
        while data_len >= MAX_PAYLOAD_LEN {
            bufp[0] = 0xff;
            bufp[1] = 0xff;
            bufp[2] = 0xff;
            bufp[3] = self.sequence;
            self.stream.write_all(&bufp[..4 + MAX_PAYLOAD_LEN]).await?;
            self.sequence += 1;
            data_len -= MAX_PAYLOAD_LEN;
            //原地拆包法
            bufp = &mut bufp[MAX_PAYLOAD_LEN..];
        }
        //the rest data
        let mut wtr = Vec::new();
        wtr.write_u24::<LE>(data_len as u32)?;
        bufp[0] = wtr[0];
        bufp[1] = wtr[1];
        bufp[2] = wtr[2];
        bufp[3] = self.sequence;
        self.stream.write_all(&bufp).await?;
        self.sequence += 1;
        Ok(())
    }
}
