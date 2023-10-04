struct MyHashMap {
    buckets: Vec<Vec<(i32,i32)>>,
}

const N_BUCKETS: usize = 1031;

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {

    fn new() -> Self {
        Self { buckets: vec![vec![]; N_BUCKETS] }

    }

    // 해쉬 생성
    fn hash(key: i32) -> usize {
        key as usize % N_BUCKETS
    }

    fn find_entry(&mut self, key: i32) -> (&mut Vec<(i32, i32)>, Result<usize, usize>) {
        let bucket = &mut self.buckets[Self::hash(key)];
        let result = bucket.binary_search_by(|(k, v)| k.cmp(&key));
        (bucket, result)
    }
    
    fn put(&mut self, key: i32, value: i32) {
         match self.find_entry(key) {
            (bucket, Ok(index)) => bucket[index] = (key, value),
            (bucket, Err(index)) => bucket.insert(index, (key, value)),
        }
    }
    
    fn get(&self, key: i32) -> i32 {
        let bucket = &self.buckets[Self::hash(key)];
        // binary_serach 사용해서 index 찾음
        match bucket.binary_search_by(|(k, v)| k.cmp(&key)) {
            Ok(index) => bucket[index].1,
            Err(index) => -1,
        }
    }
    
    fn remove(&mut self, key: i32) {
        match self.find_entry(key) {
            (bucket, Ok(index)) => { bucket.remove(index); },
            _ => (),
        }
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */