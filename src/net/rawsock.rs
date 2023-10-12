use anyhow::{anyhow, Result};
use os_socketaddr::OsSocketAddr;

use std::ffi::CStr;

use libc::close;
use libc::{c_int, c_void, size_t};
use std::net::SocketAddr;

#[link(name = "rawsock", kind = "static")]
extern "C" {
    fn get_icmp_rs(ttl: c_int, rcv_timeout_sec: c_int) -> c_int;
}

const GET_ICMP_RS_FAIL_CREATE_SOCK: c_int = -1;
const GET_ICMP_RS_FAIL_SET_TTL: c_int = -2;
const GET_ICMP_RS_FAIL_SET_TIMOUT: c_int = -3;

pub struct RawSocket {
    sock: c_int,
}

impl RawSocket {
    #[cfg(target_family = "unix")]
    pub fn new_icmp(ip_ttl: u8, timeout_sec: u32) -> Result<Self> {
        let sock = unsafe { get_icmp_rs(ip_ttl as c_int, timeout_sec as c_int) };
        if sock < 0 {
            match sock {
                GET_ICMP_RS_FAIL_CREATE_SOCK => {
                    return Err(anyhow!("fail to create socket: {}", c_error_msg()))
                }
                GET_ICMP_RS_FAIL_SET_TTL => {
                    return Err(anyhow!("fail to set TTL: {}", c_error_msg()))
                }
                GET_ICMP_RS_FAIL_SET_TIMOUT => {
                    return Err(anyhow!("fail to set responce timeout: {}", c_error_msg()))
                }
                _ => return Err(anyhow!("unknown error: {}", c_error_msg())),
            }
        }

        Ok(RawSocket { sock })
    }

    #[cfg(target_family = "unix")]
    pub fn sendto(&self, payload: &[u8], dst: SocketAddr) -> Result<usize> {
        let addr: OsSocketAddr = dst.into();
        let nb = unsafe {
            libc::sendto(
                self.sock,
                payload.as_ptr() as *const c_void,
                payload.len() as size_t,
                0,
                addr.as_ptr(),
                addr.len(),
            )
        };
        if nb < 0 {
            return Err(anyhow!("fail to send data by socket: {}", c_error_msg()));
        }
        Ok(nb as usize)
    }

    #[cfg(target_family = "unix")]
    pub fn recvfrom(&self, payload: &mut [u8]) -> Result<(usize, Option<SocketAddr>)> {
        let mut addr = OsSocketAddr::new();
        let mut addrlen = addr.capacity();
        let nb = unsafe {
            libc::recvfrom(
                self.sock,
                payload.as_mut_ptr() as *mut c_void,
                payload.len(),
                0,
                addr.as_mut_ptr(),
                &mut addrlen as *mut _,
            )
        };
        if nb < 0 {
            return Err(anyhow!(
                "fail to receive data from socket: {}",
                c_error_msg()
            ));
        }

        Ok((nb as usize, addr.into()))
    }
}

impl Drop for RawSocket {
    fn drop(&mut self) {
        unsafe {
            let code = close(self.sock);
            if code < 0 {
                panic!("fail to close the socket: {}", c_error_msg());
            }
        }
    }
}

fn c_error_msg() -> &'static str {
    let errno = std::io::Error::last_os_error().raw_os_error().unwrap();
    let err_msg = unsafe { libc::strerror(errno) };
    let err_msg: &CStr = unsafe { CStr::from_ptr(err_msg) };

    err_msg.to_str().unwrap_or("")
}
