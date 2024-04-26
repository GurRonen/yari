
// pub(crate) trait StatisticalFilter<T> 
// where T: Hash {
//     pub(crate) fn set(&mut self, item: T);
//     pub(crate) fn check(&self, item: &T);
// }

// pub(crate) struct BloomFilter<T>
// where T: Hash {
//     size: usize,
//     map: [bool]
// }

// impl BloomFilter {
//     pub(crate) fn new(size: usize) {
//         BloomFilter {
//             size,
//             map: [bool; size],
//         }
        
//     }
// }

// impl Default for BloomFilter {
//     fn default() -> Self {
//         Self::new(10)
//     }
// }

// impl StatisticalFilter for BloomFilter<T>
// where T: Hash {
//     fn set(&mut self, item: T) {
//         let x = item.hash();
//         self.map[item]
//     }

//     fn check(&self, item: &T) {
//         todo!()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use std::hash::Hash;

//     use super::*;

//     #[test]
//     fn test_set_and_check() {
//         let mut bloom: BloomFilter = BloomFilter::new(10);

//         let x = "str";
//         x.hash(123);
//         assert!(bloom.check(x));


//     }
// }