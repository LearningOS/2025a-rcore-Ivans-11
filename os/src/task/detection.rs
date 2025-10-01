use core::cmp::max;
use alloc::vec;
use alloc::vec::Vec;

/// Deadlock Detection
pub struct DeadlockDetection {
    /// max resource id
    r_max: usize,
    /// max task id
    t_max: usize,
    /// avaliable vector
    avaliable: Vec<i32>,
    /// allocation matrix
    allocation: Vec<Vec<i32>>,
    /// need matrix
    need: Vec<Vec<i32>>,
}

impl DeadlockDetection {
    /// Create a new deadlock detection
    pub fn new() -> Self {
        Self {
            r_max: 0,
            t_max: 0,
            avaliable: Vec::new(),
            allocation: Vec::new(),
            need: Vec::new(),
        }
    }
    /// Update the deadlock detection
    pub fn update(&mut self, rid: usize, tid :usize) {
        if rid <= self.r_max && tid <= self.t_max && self.r_max !=0 && self.t_max !=0 {
            return;
        }
        self.r_max = max(self.r_max, rid);
        self.t_max = max(self.t_max, tid);
        // avaliable
        while self.avaliable.len() <= self.r_max {
            self.avaliable.push(0);
        }
        // allocation
        for i in 0..self.allocation.len() {
            while self.allocation[i].len() <= self.r_max {
                self.allocation[i].push(0);
            }
        }
        while self.allocation.len() <= self.t_max {
            self.allocation.push(vec![0; self.r_max + 1]);
        }
        // need
        for i in 0..self.need.len() {
            while self.need[i].len() <= self.r_max {
                self.need[i].push(0);
            }
        }
        while self.need.len() <= self.t_max {
            self.need.push(vec![0; self.r_max + 1]);
        }
    }
    /// Add avaliable with rid
    pub fn add_avaliable(&mut self, rid: usize, count: i32) {
        self.update(rid, 0);
        self.avaliable[rid] += count;
    }
    /// Add allocation with rid and tid
    pub fn add_allocation(&mut self, rid: usize, tid: usize, count: i32) {
        self.update(rid, tid);
        self.allocation[tid][rid] += count;
    }
    /// Add need with rid and tid
    pub fn add_need(&mut self, rid: usize, tid: usize, count: i32) {
        self.update(rid, tid);
        self.need[tid][rid] += count;
    }
    /// Check if the system is in a safe state
    pub fn is_safe(&mut self) -> bool {
        let mut work = self.avaliable.clone();
        let mut finish = vec![false; self.t_max + 1];
        let mut count = 0;
        while count <= self.t_max {
            let mut found = false;
            for i in 0..self.t_max + 1 {
                if !finish[i] {
                    let mut valid = true;
                    let mut j = 0;
                    while j <= self.r_max {
                        if self.need[i][j] > work[j] {
                            valid = false;
                            break;
                        }
                        j += 1;
                    }
                    if valid {
                        found = true;
                        finish[i] = true;
                        let mut j = 0;
                        while j <= self.r_max {
                            work[j] += self.allocation[i][j];
                            j += 1;
                        }
                        break;
                    }
                }
            }
            if !found {
                break;
            }
            count += 1;
        }
        count == self.t_max + 1
    }
}