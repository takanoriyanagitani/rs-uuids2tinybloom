use std::io;

use io::BufRead;

use io::ErrorKind;

use io::Write;

pub const TINY_BLOOM_BYTE_SIZE: u8 = 32;
pub const TINY_BLOOM_BIT_SIZE: u16 = 256;

pub const NUM_OF_ITEMS_DEFAULT: u8 = TINY_BLOOM_BYTE_SIZE;
pub const NUM_OF_HASH_DEFAULT: u8 = 4;
pub const HASH_SIZE: u8 = 1;

pub fn uuid2hash4_hi(u: u128) -> [u8; 4] {
    let a: [u8; 16] = u.to_be_bytes();
    [a[0], a[1], a[2], a[3]]
}

pub fn uuid2hash4(u: u128) -> [u8; 4] {
    uuid2hash4_hi(u)
}

pub fn set_bit(mut bloom: [u8; 32], bitpos: u8) -> [u8; 32] {
    let bytepos: u8 = bitpos >> 3;
    let original: u8 = bloom[bytepos as usize];
    let localpos: u8 = bitpos & 0x07;
    let one: u8 = 1;
    let shifted: u8 = one << localpos;
    let neo: u8 = original | shifted;
    bloom[bytepos as usize] = neo;
    bloom
}

pub fn update_bloom(bloom: [u8; 32], id: u128) -> [u8; 32] {
    let hash: [u8; 4] = uuid2hash4(id);
    hash.into_iter().fold(bloom, |state, next| {
        let current_hash: u8 = next;
        set_bit(state, current_hash)
    })
}

pub fn uuids2bloom<I>(uuids: I) -> [u8; 32]
where
    I: Iterator<Item = u128>,
{
    uuids.fold([0; 32], update_bloom)
}

pub fn rdr2uuids_raw<R>(mut rdr: R) -> impl Iterator<Item = Result<u128, io::Error>>
where
    R: BufRead,
{
    let mut buf: [u8; 16] = [0; 16];

    std::iter::from_fn(move || match rdr.read_exact(&mut buf) {
        Ok(_) => Some(Ok(u128::from_be_bytes(buf))),
        Err(e) => match e.kind() {
            ErrorKind::UnexpectedEof => None,
            _ => Some(Err(e)),
        },
    })
}

pub fn stdin2uuids_raw() -> impl Iterator<Item = u128> {
    rdr2uuids_raw(io::stdin().lock()).map_while(Result::ok)
}

pub fn bloom2wtr_raw<W>(mut wtr: W) -> impl FnMut([u8; 32]) -> Result<(), io::Error>
where
    W: Write,
{
    move |bloom: [u8; 32]| {
        let s: &[u8] = &bloom;
        wtr.write_all(s)
    }
}

pub fn bloom2stdout_raw(bloom: [u8; 32]) -> Result<(), io::Error> {
    let o = io::stdout();
    let mut l = o.lock();
    bloom2wtr_raw(&mut l)(bloom)?;
    l.flush()
}

pub fn stdin2uuids2bloom2stdout_raw() -> Result<(), io::Error> {
    let uuids = stdin2uuids_raw();
    let bloom: [u8; 32] = uuids2bloom(uuids);
    bloom2stdout_raw(bloom)
}
