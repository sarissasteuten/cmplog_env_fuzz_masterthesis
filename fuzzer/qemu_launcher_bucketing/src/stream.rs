// byte stream layout
use crate::syscalls;
use std::cell::RefCell;
use std::collections::HashMap;

pub const BUCKET_SIZE: usize = 1000;
pub const N_BUCKETS: usize = syscalls::N_SYSCALLS;
pub const STREAM_SIZE: usize = BUCKET_SIZE * N_BUCKETS;

#[derive(Default)]
pub struct Bucket {
    pub bytes: Vec<u8>,
    pub position: usize,
}
#[derive(Default)]
pub struct MapBuckets {
    pub buckets: HashMap<i32, Bucket>,
}

pub fn set_seed(path: &str) {
    let mut buf = vec![0u8; STREAM_SIZE];

    for byte in buf.iter_mut() {
        *byte = rand::random::<u8>();
    }
    std::fs::write(path, buf).expect("cannot writeeee seed\n");
}

thread_local! {
    pub static CURRENT_BUCKETS: RefCell<MapBuckets> = RefCell::new(MapBuckets::default());
}

pub fn set_current_stream(b: &[u8]) {
    CURRENT_BUCKETS.with(|bucket_map: &RefCell<MapBuckets>| {
        let mut bucket_map = bucket_map.borrow_mut();
        bucket_map.buckets.clear();
        // println!("NEW INPUT SIZE: {}", b.len());
        for (i, syscall) in syscalls::SYSCALLS.iter().enumerate() {
            let start = i * BUCKET_SIZE;
            let end = start + BUCKET_SIZE;
            let end_safe = end.min(b.len());

            let mut bytes_vec = if start >= b.len() {
                vec![0u8; BUCKET_SIZE]
            } else {
                b[start..end_safe].to_vec()
            };

            bytes_vec.resize(BUCKET_SIZE, 0);
            bucket_map.buckets.insert(
                syscall.num,
                Bucket {
                    bytes: bytes_vec,
                    position: 0,
                },
            );
        }
    });
}

pub fn consume_bytes(syscall: i32, size: usize) -> Vec<u8> {
    CURRENT_BUCKETS.with(|b| {
        
        let mut b = b.borrow_mut();
        let bucket = b.buckets.get_mut(&syscall).expect("cant find syscallllll");

        if size == 0 {
            return Vec::new();
        }

        let len = bucket.bytes.len();
        let start_pos = bucket.position % len;
        let end_pos = start_pos + size;

        if end_pos <= len {
            bucket.position = end_pos % len;
            return bucket.bytes[start_pos..end_pos].to_vec();
        }

        let mut temp_vec = Vec::with_capacity(size);

        temp_vec.extend_from_slice(&bucket.bytes[start_pos..len]);

        let mut left = size - (len - start_pos);
        while left >= len {
            temp_vec.extend_from_slice(&bucket.bytes[..]);
            left -= len;
        }

        if left > 0 {
            temp_vec.extend_from_slice(&bucket.bytes[0..left]);
        }

        bucket.position = (start_pos + size) % len;
        temp_vec
    })
}
