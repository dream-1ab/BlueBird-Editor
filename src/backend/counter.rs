use serde::{Deserialize, Serialize};

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-09-12 15:05:24
 * @modify date 2024-09-12 15:05:24
 * @desc [description]
*/


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Counter {
    pub count: usize
}

impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn count(&mut self) -> usize {
        self.count += 1;
        self.count
    }
}
