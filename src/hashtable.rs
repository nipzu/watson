use std::sync::atomic::{AtomicU32, Ordering};

use crate::evaluation::Evaluation;

pub struct HashTable {
    num_buckets: usize,
    bucket_size: usize,
    table: Vec<AtomicU32>,
}

#[allow(clippy::cast_sign_loss)]
const EMPTY_TABLE_ENTRY: u32 = (Evaluation::RESERVED_VALUE as u32) << 16;

impl HashTable {
    pub fn new(order_buckets: u32, bucket_size: usize) -> Self {
        let num_buckets = 2_usize.pow(order_buckets);
        let mut table = Vec::with_capacity(num_buckets * bucket_size);

        for _ in 0..num_buckets * bucket_size {
            table.push(AtomicU32::new(EMPTY_TABLE_ENTRY));
        }

        Self {
            num_buckets,
            bucket_size,
            table,
        }
    }

    pub fn clear(&mut self) {
        for a in &self.table {
            a.store(EMPTY_TABLE_ENTRY, Ordering::Relaxed);
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn get_eval(&self, hash: u64) -> Option<Evaluation> {
        let low_bits = (hash as usize) & (self.num_buckets - 1);
        let high_bits = (hash >> 48) as u16;

        let base_index = low_bits * self.bucket_size;
        for i in 0..self.bucket_size {
            let entry = self.table[base_index + i].load(Ordering::Relaxed);
            if (entry as u16) == high_bits {
                return Some(Evaluation::from_raw((entry >> 16) as i16));
            }
        }

        None
    }

    /// TODO: test `compare_exchange` performance
    #[allow(clippy::cast_possible_truncation)]
    pub fn insert_eval(&self, hash: u64, eval: Evaluation) {
        let low_bits = (hash as usize) & (self.num_buckets - 1);
        // the 16 most significant bits of the hash
        let high_bits = (hash >> 48) as u32;

        let base_index = low_bits * self.bucket_size;
        for i in 0..self.bucket_size {
            let entry = self.table[base_index + i].load(Ordering::Relaxed);
            if entry == EMPTY_TABLE_ENTRY {
                #[allow(clippy::cast_sign_loss)]
                let new_entry = high_bits | (u32::from(eval.to_raw() as u16) << 16);
                self.table[base_index + i].store(new_entry, Ordering::Relaxed);
                return;
            }
        }
    }
}
