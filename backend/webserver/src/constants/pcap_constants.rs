

#[derive(Debug)]
pub enum PcapSizes {
    Free,
    Paid,
    SmallBusiness,
    Enterprise,
}

impl PcapSizes {

    pub fn max_bytes(&self) -> usize{

        match self {
            PcapSizes::Free => 5 * 1024 * 1024,
            PcapSizes::Paid => 50 * 1024 * 1024,
            PcapSizes::SmallBusiness => 200 * 1024 * 1024,
            PcapSizes::Enterprise => 1024 * 1024 * 1024,
        }
    }
}

