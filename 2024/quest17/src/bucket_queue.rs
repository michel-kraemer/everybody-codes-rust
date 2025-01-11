pub struct BucketQueue<T> {
    queue: Vec<Vec<T>>,
    last_bucket: usize,
}

impl<T> BucketQueue<T>
where
    T: Clone,
{
    pub fn new(len: usize) -> Self {
        Self {
            queue: vec![Vec::new(); len],
            last_bucket: 0,
        }
    }

    pub fn insert(&mut self, bucket: usize, item: T) {
        self.queue[bucket].push(item);
        self.last_bucket = self.last_bucket.min(bucket);
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut i = self.last_bucket;
        while i < self.queue.len() && self.queue[i].is_empty() {
            i += 1;
        }
        self.last_bucket = i;
        if i == self.queue.len() {
            None
        } else {
            Some(self.queue[i].swap_remove(0))
        }
    }
}
