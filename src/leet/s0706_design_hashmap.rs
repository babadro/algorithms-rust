const CAPACITY: usize = 10_000;

struct MyHashMap {
    storage: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    fn new() -> Self {
        MyHashMap {
            storage: vec![Vec::default(); CAPACITY],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let idx = hash(key);

        for i in (0..self.storage[idx].len()).step_by(2) {
            if self.storage[idx][i] == key {
                self.storage[idx][i + 1] = value;

                return;
            }
        }

        self.storage[idx].push(key);
        self.storage[idx].push(value);
    }

    fn get(&self, key: i32) -> i32 {
        let idx = hash(key);

        for i in (0..self.storage[idx].len()).step_by(2) {
            if self.storage[idx][i] == key {
                return self.storage[idx][i + 1];
            }
        }

        -1
    }

    fn remove(&mut self, key: i32) {
        let idx = hash(key);

        for i in (0..self.storage[idx].len()).step_by(2) {
            if self.storage[idx][i] == key {
                self.storage[idx].swap_remove(i + 1);
                self.storage[idx].swap_remove(i);

                return;
            }
        }
    }
}

fn hash(key: i32) -> usize {
    key as usize % CAPACITY
}
