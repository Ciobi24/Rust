use std::io;
use std::collections::HashMap;
use std::cell::RefCell;

struct Cache {
    cache: RefCell<HashMap<u64, bool>>,
    recent_numbers: RefCell<Vec<u64>>,
}

impl Cache {
    fn new() -> Cache {
        Cache {
            cache: RefCell::new(HashMap::new()),
            recent_numbers: RefCell::new(Vec::new()),
        }
    }

    fn is_prime(&self, num: u64) -> bool {
        if num <= 1 {
            return false;
        }

        for i in 2..=(num as f64).sqrt() as u64 {
            if num % i == 0 {
                return false;
            }
        }
        true
    }

    fn get_from_cache(&self, num: u64) -> Option<bool> {
        self.cache.borrow().get(&num).cloned()
    }

    fn insert_into_cache(&self, num: u64, is_prime: bool) {
        let mut cache = self.cache.borrow_mut();
        let mut recent_numbers = self.recent_numbers.borrow_mut();

        if recent_numbers.len() >= 10 {
            if let Some(oldest) = recent_numbers.pop() {
                cache.remove(&oldest);
            }
        }

        cache.insert(num, is_prime);
        recent_numbers.insert(0, num);
    }
}

fn main() {
    let cache = Cache::new();

    loop {
        println!("Enter a number: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if let Ok(num) = input.trim().parse::<u64>() {
            if let Some(is_prime) = cache.get_from_cache(num) {
                println!("Cached Result: {} is prime: {}", num, is_prime);
            } else {
                let is_prime = cache.is_prime(num);
                cache.insert_into_cache(num, is_prime);
                println!("New Result: {} is prime: {}", num, is_prime);
            }
        } else {
            println!("Invalid input. Please enter a number.");
        }
    }
}
