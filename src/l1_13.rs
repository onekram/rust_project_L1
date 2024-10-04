use core::hash;
use std::io;

struct MyHashSetString {
    buckets: Vec<Vec<String>>,
    size: usize,
}

impl MyHashSetString {
    fn new() -> Self {
        MyHashSetString {
            buckets: vec![Vec::new(); 3],
            size: 0,
        }
    }

    fn add(&mut self, el: &str) {
        let hash = self.hash(el);
        
        if !self.buckets[hash].contains(&el.to_string()) {
            self.buckets[hash].push(el.to_string());
            self.size += 1;
            if self.size >= self.buckets.len() * 2 {
                self.resize();
            }
        }
    }

    fn contains(&self, el: &str) -> bool {
        let hash = self.hash(el);
        self.buckets[hash].contains(&el.to_string())
    }

    fn len(&self) -> usize {
        self.size
    }

    fn hash(&self, el: &str) -> usize {
        let mut res: usize = 0;
        for (i, b) in el.bytes().enumerate() {
            res = res.wrapping_add(b as usize).wrapping_mul(i);
        }
        return res % self.buckets.len();
    }

    fn resize(&mut self) {
        let new_size = self.buckets.len() * 2;
        let mut new_buckets = vec![Vec::new(); new_size];
        for bucket in &self.buckets {
            for el in bucket {
                new_buckets[self.hash(el)].push(el.to_string());
            }
        }
        self.buckets = new_buckets;
        println!("{}", self.buckets.len());
    }
}

struct MyHashSetStringIterator<'a> {
    set: &'a MyHashSetString,
    bucket_index: usize,
    string_index: usize,
}

impl<'a> Iterator for MyHashSetStringIterator<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        while self.bucket_index < self.set.buckets.len() {
            if self.string_index < self.set.buckets[self.bucket_index].len() {
                let result = &self.set.buckets[self.bucket_index][self.string_index];
                self.string_index += 1;
                return Some(result);
            } else {
                self.bucket_index += 1;
                self.string_index = 0;
            }
        }
        None
    }
}

impl<'a> IntoIterator for &'a MyHashSetString {
    type Item = &'a String;
    type IntoIter = MyHashSetStringIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MyHashSetStringIterator {
            set: self,
            bucket_index: 0,
            string_index: 0,
        }
    }
}


pub fn unique_string() {
    let mut input = String::new();

    let mut set = MyHashSetString::new();

    while io::stdin().read_line(&mut input).expect("Error reading input") > 0 {
        let trimmed = input.trim().to_string();
        input.clear();

        if trimmed.eq_ignore_ascii_case("exit") {
            break;
        }
        set.add(&trimmed);
    }

    for el in &set {
        println!("{}", el);
    }
}
