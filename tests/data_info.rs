// Dummy struct for the purposes of testing
pub struct DataInfo {
    pub size: u64,
    pub data: i32
}

// Sample trait implementation - Easy!
impl packer::Pack for DataInfo {
    fn get_size(&self) -> u64 {
        self.size
    }
}