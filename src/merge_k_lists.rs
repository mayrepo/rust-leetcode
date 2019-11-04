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

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        None
    }
}

struct Test {
    lists: Vec<Option<Box<ListNode>>>,
}

impl std::iter::Iterator for Test {
    
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        let (index, _) = self.lists.iter().enumerate().filter(|(_, x)| x.is_some()).map(|(i, x)| (i, x.unwrap())).min_by(|(i,x), (j, y)| x.val.cmp(&y.val))?;
        
        let list = self.lists.remove(index).unwrap();
        
        let next = list.val;
        
        self.lists.push(Box::new(list.next));
        
        next
    }
}
