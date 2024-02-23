use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_uncommitted_ranges() {
        let confirmed_blocks = vec![1, 2, 3, 8, 9, 10];
        let expected = vec![BlockRange::InclusiveRange(4, 7)];

        let result = find_uncommitted_ranges(confirmed_blocks, 1, 10);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_uncommitted_ranges_no_gaps() {
        let confirmed_blocks = vec![1, 2, 3, 4, 5];
        let expected: Vec<BlockRange> = Vec::new();

        let result = find_uncommitted_ranges(confirmed_blocks, 1, 5);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_uncommitted_ranges_start_after_confirmed() {
        let confirmed_blocks = vec![3, 4, 5];
        let expected = vec![BlockRange::InclusiveRange(1, 2)];

        let result = find_uncommitted_ranges(confirmed_blocks, 1, 5);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_uncommitted_ranges_end_before_confirmed() {
        let confirmed_blocks = vec![1, 2, 3];
        let expected = vec![BlockRange::InclusiveRange(4, 10)];

        let result = find_uncommitted_ranges(confirmed_blocks, 1, 10);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_uncommitted_ranges_no_confirmed() {
        let confirmed_blocks: Vec<u64> = Vec::new();
        let expected = vec![BlockRange::InclusiveRange(1, 10)];

        let result = find_uncommitted_ranges(confirmed_blocks, 1, 10);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_block_range_new() {
        let single = BlockRange::new(1, 1);
        assert_eq!(single, BlockRange::SingleBlock(1));

        let range = BlockRange::new(1, 5);
        assert_eq!(range, BlockRange::InclusiveRange(1, 5));
    }

    #[test]
    #[should_panic]
    fn test_block_range_new_invalid() {
        BlockRange::new(5, 1);
    }

    #[test]
    fn test_block_range_display() {
        let single = BlockRange::SingleBlock(5);
        assert_eq!(format!("{single}"), "single block: 5");

        let range = BlockRange::InclusiveRange(1, 5);
        assert_eq!(format!("{range}"), "inclusive range: [1; 5]");
    }
}

