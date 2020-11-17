mod data_info;

#[cfg(test)]
mod tests {
    use crate::data_info::DataInfo;
    use packer::{Bin, Pack, online_nf, ffd};

    #[test]
    fn test_valid_nf () {
        let sample_data = generate_items(1, 100, 1000);
        assert!(sample_data.len() == 1000);

        // This shouldn't panic
        let bins: Vec<Bin<DataInfo>> = online_nf(sample_data, 100).unwrap();

        let mut sorted_items = 0;

        for bin in bins.iter() {
            let occupied_space: u64 = bin.items.iter().map(|i| i.get_size()).sum();
            assert!(occupied_space + bin.remaining_space == 100);
            sorted_items += bin.items.len();
        }

        // All items are sorted?
        assert!(sorted_items == 1000)
    }

    #[test]
    fn test_valid_ffd () {
        let sample_data = generate_items(1, 100, 1000);
        assert!(sample_data.len() == 1000);

        // This shouldn't panic
        let bins: Vec<Bin<DataInfo>> = ffd(sample_data, 100).unwrap();

        let mut sorted_items = 0;

        for bin in bins.iter() {
            let occupied_space: u64 = bin.items.iter().map(|i| i.get_size()).sum();
            assert!(occupied_space + bin.remaining_space == 100);
            sorted_items += bin.items.len();
        }

        // All items are sorted?
        assert!(sorted_items == 1000)
    }

    // A bin can be made from an existing item that should fit, and the size is correct
    #[test]
    fn make_add_bin() {
        let mut my_bin: Bin<DataInfo> = Bin::new_from(DataInfo {
            size: 60,
            data: -4
        }, 100u64);

        // Item fit successfully
        assert!(my_bin.remaining_space == 40);

        // Doesn't panic
        my_bin.add_item(DataInfo {
            size: 12,
            data: 37
        });
    }

    #[test]
    #[should_panic]
    fn make_with_too_big() {
        Bin::new_from(DataInfo {
            size: 60,
            data: -4
        }, 50u64);
    }

    fn generate_items(min: u64, max: u64, count: u64) -> Vec<DataInfo> {
        let mut items: Vec<DataInfo> = Vec::new();

        for _ in 0..count {
            items.push(DataInfo {
                size: rand::random::<u64>()%(max-min+1) + min,
                data: rand::random::<i32>()
            })
        }

        return items;
    }
}