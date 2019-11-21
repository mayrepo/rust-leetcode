use std::fmt;
use std::rc::Rc;
use std::collections::HashMap;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        
        //HashMaps mapping the minimum and maximum of sequences to the sequences (referenced by counting pointer reference (Rc))
        let mut mins = HashMap::new();
        let mut maxs = HashMap::new();
        
        for num in nums {
            if !maxs.contains_key(&num) && !mins.contains_key(&num) {
                
                //Before:
                //[min1   max1] num [min2   max2]
                //
                //After:
                //[min                       max]
                
                //If one or two of the sides do not exist, we replace max or min by num
                
                //The case where num falls into an existing sequence, it will create a new sequence that will remain strictly inside the other one because of the previous condition.
                //There could even be a tree of sequences where parents nodes contain children nodes.
                
                //By contruction two sequences can not overlap except for the case of inclusion.
                //The final result would remain unaffected.
                
                let min = maxs.remove(&(num-1)).map(|s: Rc<Sequence>| s.min).unwrap_or(num);
                let max = mins.remove(&(num+1)).map(|s: Rc<Sequence>| s.max).unwrap_or(num);

                let new_sequence = Rc::new(Sequence{min, max});
                mins.insert(min, new_sequence.clone());
                maxs.insert(max, new_sequence.clone());
            }
            
        }
        mins.iter().map(|(_, s)| s.max-s.min+1 ).max().unwrap_or(0)
    }
}

struct Sequence {
    min: i32,
    max: i32,
}

impl fmt::Debug for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.min==self.max {
            write!(f, "{}", self.min)
        } else {
            write!(f, "{},{}", self.min, self.max)
        }
    }
}
