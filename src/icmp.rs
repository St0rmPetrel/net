use anyhow::{anyhow, Result};

const HDR_BYTE_SIZE: usize = 8;

const ECHO_REPLY_TYPE: u8 = 0;
const ECHO_REPLY_CODE: u8 = 0;
const ECHO_REQUEST_TYPE: u8 = 8;
const ECHO_REQUEST_CODE: u8 = 0;

#[repr(C)]
pub struct Packet<const DATA_SIZE: usize> {
    p_type: u8,
    code: u8,
    checksum: u16,
    rest_hdr: RestHdr,
    data: [u8; DATA_SIZE],
}

#[repr(C)]
union RestHdr {
    // echo datagram
    echo: Echo,
    // gateway address
    gateway: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// echo datagram
struct Echo {
    id: u16,
    sequence: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// path mtu discovery
struct Frag {
    __unused: u16,
    mtu: u16,
}

impl<const DATA_SIZE: usize> Packet<DATA_SIZE> {
    pub fn new_echo_req(id: u16, sequence: u16, data: &[u8; DATA_SIZE]) -> Packet<DATA_SIZE> {
        let mut pck = Packet {
            p_type: ECHO_REQUEST_TYPE,
            code: ECHO_REQUEST_CODE,
            checksum: 0x00,
            rest_hdr: RestHdr {
                echo: Echo { id, sequence },
            },
            data: [0x7; DATA_SIZE],
        };
        pck.data.clone_from_slice(data);
        pck.checksum();

        return pck;
    }

    pub fn from_raw(raw_pck: &[u8]) -> Result<Packet<DATA_SIZE>> {
        if raw_pck.len() < HDR_BYTE_SIZE {
            return Err(anyhow!("packet size is to small"));
        }
        let p_type = raw_pck[0];
        let code = raw_pck[1];
        let expected_checksum: u16 = to_u16(raw_pck[2], raw_pck[3]);

        let mut pck = match p_type {
            ECHO_REPLY_TYPE | ECHO_REQUEST_TYPE => Packet {
                p_type,
                code,
                checksum: 0,
                rest_hdr: RestHdr {
                    echo: Echo {
                        id: to_u16(raw_pck[5], raw_pck[4]),
                        sequence: to_u16(raw_pck[7], raw_pck[6]),
                    },
                },
                data: [0; DATA_SIZE],
            },
            _ => unimplemented!(),
        };
        pck.data.clone_from_slice(&raw_pck[HDR_BYTE_SIZE..]);

        pck.checksum();
        let actual_checksum = u16_switch_endian(pck.checksum);

        if actual_checksum != expected_checksum {
            return Err(anyhow!(
                "checksum is incorrect: actual = {:X?}, expect = {:X?}",
                actual_checksum,
                expected_checksum,
            ));
        }

        Ok(pck)
    }

    pub fn raw(&self) -> &[u8] {
        let ptr = &self.p_type as *const u8;
        unsafe { std::slice::from_raw_parts(ptr, HDR_BYTE_SIZE + DATA_SIZE) }
    }

    fn checksum(&mut self) {
        self.checksum = 0x0000;
        let mut sum: u32 = 0;

        for w in self.raw().chunks(2) {
            let word = to_u16(w[0], w[1]);

            sum += word as u32;

            let carry = (0xFFFF_0000 & sum) >> 16;
            sum = sum & 0x0000_FFFF;
            sum += carry;
        }

        let carry = (0xFFFF_0000 & sum) >> 16;
        sum = sum & 0x0000_FFFF;
        sum += carry;

        sum = !sum;

        self.checksum = u16_switch_endian(sum as u16);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn checksum() {
        let pkt = Packet::<32>::new_echo_req(
            0x0100,
            0x0810,
            &[
                0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e,
                0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x61, 0x62, 0x63, 0x64, 0x65,
                0x66, 0x67, 0x68, 0x69,
            ],
        );

        println!("want checksum = {:0>4x?}", 0x3d53);
        println!("actual checksum = {:0>4x?}", pkt.checksum);
        println!("raw packet = {:0>2x?}", pkt.raw());

        // 0x3d53 BE
        assert!(pkt.checksum == 0x533d)
    }

    #[test]
    fn from_raw_positive() {
        let raw_pck: [u8; 40] = [
            0x08, 0x00, 0x3d, 0x53, 0x00, 0x01, 0x10, 0x08, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66,
            0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74,
            0x75, 0x76, 0x77, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69,
        ];

        let pck = Packet::<32>::from_raw(&raw_pck);

        pck.unwrap();
    }
}

fn to_u16(r: u8, l: u8) -> u16 {
    let mut ret: u16 = r as u16;
    ret = ret << 8;
    ret += l as u16;

    ret
}

fn u16_switch_endian(x: u16) -> u16 {
    let mut ret = (x & 0x00FF) << 8;
    ret += (x & 0xFF00) >> 8;

    ret
}
