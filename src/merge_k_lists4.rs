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
        
        let mut root_node=None;
        
        //Populate the tree (owned by root_node) by adding every new value yielded by the iterator at the end of the tree (mutably borrowed by last_node)  
        Iterator::new(lists)
            .fold(&mut root_node, |last_node, i| {
                *last_node=Some(Box::new(ListNode {next: None, val: i}));
                &mut last_node.as_mut().unwrap().next
            });
        root_node
    }
}

struct Iterator {
    lists: BinaryHeap<ListNode>,
}

impl Iterator {
    //Populate the BinaryHeap containing the lists, ordering them by the Order defined below
    fn new(lists: Vec<Option<Box<ListNode>>>) -> Iterator {
        Iterator {lists: lists.into_iter().filter_map(|a| a).map(|l| *l).collect()}
    }
}

impl std::iter::Iterator for Iterator {
    type Item = i32;
    
    fn next(&mut self) -> Option<i32> {
        
        //Pop the List with the lowest first element
        let ListNode {val, next} = self.lists.pop()?;
        
        match next {
            None => {},
            //Add the remaining of the list the BinaryHeap
            Some(next) => self.lists.push(ListNode {val: next.val, next: next.next}),
        }
        Some(val)
    }
}

//Invered order to get a minimum BinaryHeap because the standard BinaryHeap is a maximum BinaryHeap
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

