// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

use std::cmp::Ordering;
use std::collections::BinaryHeap;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        Iterator::new(lists)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .fold(None, |result, i| {
                Some(Box::new(ListNode {next: result, val: i}))
            })
    }
}

struct Iterator {
    lists: BinaryHeap<ListNode>,
}

impl Iterator {
    fn new(lists: Vec<Option<Box<ListNode>>>) -> Iterator {
        Iterator {lists: lists.into_iter().filter_map(|a| a).map(|l| *l).collect()}
    }
}

impl std::iter::Iterator for Iterator {
    type Item = i32;
    
    fn next(&mut self) -> Option<i32> {
        
        let ListNode {val, next} = self.lists.pop()?;
        
        match next {
            None => {},
            Some(next) => self.lists.push(ListNode {val: next.val, next: next.next}),
        }
        Some(val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

