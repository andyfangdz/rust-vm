extern crate bytes;
extern crate protobuf;

use self::bytes::Bytes;
use self::bytes::BytesMut;

pub struct ROM {
    data: Bytes,
}

pub fn make_rom(data: Bytes) -> ROM {
    ROM { data }
}

pub struct RAM {
    data: BytesMut,
}

pub fn make_ram(capacity: usize) -> RAM {
    RAM {
        data: BytesMut::with_capacity(capacity),
    }
}
