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
        
        let mut result = None;
        
        for i in (Iterator::new(lists)).collect::<Vec<_>>().iter().rev() {
            result = Some(Box::new(ListNode {next: result, val: *i}));
        }
        result
    }
}

struct Iterator {
    lists: std::collections::BinaryHeap<Node>,
}

impl Iterator {
    #[inline]
    fn new(lists: Vec<Option<Box<ListNode>>>) -> Iterator {
        Iterator {lists: lists.into_iter().filter_map(|a| a).map(|list| Node {val: list.val, next: list.next}).collect()}
    }
}

impl std::iter::Iterator for Iterator {
    type Item = i32;
    
    #[inline]
    fn next(&mut self) -> Option<i32> {
        
        let Node {val, next} = self.lists.pop()?;
        
        match next {
            None => {},
            Some(next) => self.lists.push(Node {val: next.val, next: next.next}),
        }
        Some(val)
    }
}

#[derive(Eq)]
struct Node {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl Ord for Node {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val).reverse()
    }
}

impl PartialOrd for Node {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}
