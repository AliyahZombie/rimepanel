//! Minimal Server List Ping client (players online/max).
use anyhow::{bail, Result};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

fn write_varint(buf: &mut Vec<u8>, mut v: i32) {
    loop {
        let mut b = (v & 0x7f) as u8;
        v = ((v as u32) >> 7) as i32;
        if v != 0 {
            b |= 0x80;
        }
        buf.push(b);
        if v == 0 {
            break;
        }
    }
}

async fn read_varint(s: &mut TcpStream) -> Result<i32> {
    let mut out = 0i32;
    for i in 0..5 {
        let b = s.read_u8().await?;
        out |= ((b & 0x7f) as i32) << (7 * i);
        if b & 0x80 == 0 {
            return Ok(out);
        }
    }
    bail!("varint too long")
}

#[allow(dead_code)]
pub struct Status {
    pub online: u32,
    pub max: u32,
    pub version: String,
}

pub async fn ping(port: u16) -> Result<Status> {
    tokio::time::timeout(Duration::from_secs(3), ping_inner(port)).await?
}

async fn ping_inner(port: u16) -> Result<Status> {
    let mut s = TcpStream::connect(("127.0.0.1", port)).await?;
    let mut body = Vec::new();
    write_varint(&mut body, 0x00);
    write_varint(&mut body, -1);
    let host = b"127.0.0.1";
    write_varint(&mut body, host.len() as i32);
    body.extend_from_slice(host);
    body.extend_from_slice(&port.to_be_bytes());
    write_varint(&mut body, 1);
    let mut pkt = Vec::new();
    write_varint(&mut pkt, body.len() as i32);
    pkt.extend(body);
    pkt.extend_from_slice(&[0x01, 0x00]);
    s.write_all(&pkt).await?;
    let _len = read_varint(&mut s).await?;
    let id = read_varint(&mut s).await?;
    if id != 0 {
        bail!("unexpected packet");
    }
    let slen = read_varint(&mut s).await? as usize;
    if slen > 1 << 21 {
        bail!("status too large");
    }
    let mut buf = vec![0u8; slen];
    s.read_exact(&mut buf).await?;
    let v: serde_json::Value = serde_json::from_slice(&buf)?;
    Ok(Status {
        online: v["players"]["online"].as_u64().unwrap_or(0) as u32,
        max: v["players"]["max"].as_u64().unwrap_or(0) as u32,
        version: v["version"]["name"].as_str().unwrap_or("").to_string(),
    })
}
