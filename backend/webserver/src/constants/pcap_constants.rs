

#[derive(Debug)]
pub enum PcapSizes {
    Free,
    Paid,
    Business,
    Enterprise,
}

impl PcapSizes {

    pub fn max_bytes(&self) -> usize{

        match self {
            PcapSizes::Free => 5 * 1024 * 1024,
            PcapSizes::Paid => 50 * 1024 * 1024,
            PcapSizes::Business => 200 * 1024 * 1024,
            PcapSizes::Enterprise => 1024 * 1024 * 1024,
        }
    }
}


pub const PCAPNG_BYTES: [u8; 4] = [0x0A, 0x0D, 0x0D, 0x0A];