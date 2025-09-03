trait Sortable {
    fn quick_sort(&mut self);
}

/**
  * 三数取中，插排，聚集相等元数，尾递归
  */
impl Sortable for Vec<i32> {

    fn buble_sort(&mut self) {
        let len = self.len();
        for i in 0..len {
            for j in 0..(len-i-1) {
                if self[j] > self[j+1] {
                    let tmp = self[j];
                    self[j] = self[j+1];
                    self[j+1] = tmp;
                }
            }
        }
    }

    fn selection_sort(&mut self) {
        let len = self.len();
        for i in 0..len {
            let mut min_index = i;
            for j in (i+1)..len {
                if self[j] < self[min_index] {
                    min_index = j;
                }
            }
            if min_index != i {
                let tmp = self[i];
                self[i] = self[min_index];
                self[min_index] = tmp;
            }
        }
    }

    fn insertion_sort(&mut self) {
        let len = self.len();
        for i in 1..len {
            let key = self[i];
            let mut j = i as i32 - 1;
            while j >= 0 && self[j as usize] > key {
                self[(j+1) as usize] = self[j as usize];
                j -= 1;
            }
            self[(j+1) as usize] = key;
        }
    }

    fn quick_sort(&mut self) {
        if self.len() <= 1 {
            return;
        }
        let pivot = &self[0];
        // let mut left: Vec<i32> = Vec::new();
        // let mut right: Vec<i32> = Vec::new();
        for i in 1..self.len() {
            if self[i] < *pivot {
                let tmp = self[i];
                self[i] = *pivot;
                *pivot = tmp;
            } else {
                right.push(self[i]);
            }
        }
        left.quick_sort();
        right.quick_sort();
        left.push(pivot);
        left.append(&mut right);
        *self = left;
    }
}