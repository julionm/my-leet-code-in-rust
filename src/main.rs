use std::clone::Clone;

fn main () {
    
}

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let total_nums = nums1.len() + nums2.len();
    let median_position = ((total_nums as f64 / 2.0).ceil()) as usize;

    if nums1.len() == 0 {
        if total_nums % 2 == 0 {
            return (nums2[median_position] + nums2[median_position - 1]) as f64 / 2.0    
        } else {
            return nums2[median_position - 1] as f64
        }
    } else if nums2.len() == 0 {
        if total_nums % 2 == 0 {
            return (nums1[median_position] + nums1[median_position - 1]) as f64 / 2.0    
        } else {
            return nums1[median_position - 1] as f64
        }
    }

    let mut arr1 = &nums1[..];
    let mut arr2 = &nums2[..];

    let mut res: Vec<i32> = Vec::new();

    for _ in 0..median_position + 1 {
        if arr1[0] < arr2[0] {
            res.push(arr1[0]);

            if arr1.len() > 1 {
                arr1 = &arr1[1..];
            } else {
                res.append(&mut arr2.to_vec());
                break;
            }

        } else {
            res.push(arr2[0]);

            if arr2.len() > 1 {
                arr2 = &arr2[1..];
            } else {
                res.append(&mut arr1.to_vec());
                break;
            }
        }  
    }

    if total_nums % 2 == 0 {
        (res[median_position] + res[median_position - 1]) as f64 / 2.0    
    } else {
        res[median_position - 1] as f64
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn add_two_numbers_version1(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn sum_linked_list_val(mut list: Option<Box<ListNode>>) -> i32 {
        let mut l1_val = 0;

        let mut acc = 0;

        loop {
            if let Some(ref node) = list {
                l1_val += (**node).val * (10i32.pow(acc));
            }

            acc += 1;

            list = match list {
                Some(x) => x.next,
                None => None
            };

            if let None = list {
                break;
            }
        }

        l1_val
    }

    let result: i32 = sum_linked_list_val(l1) + sum_linked_list_val(l2);

    let mut arr: Vec<i32> = result.to_string()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect();
    
    arr.reverse();

    let mut head = Some(Box::new(ListNode::new(arr[0])));
    let mut previous = &mut head;
    
    for (index, number) in arr[1..].iter().enumerate() {            
        let aux = Some(Box::new(ListNode::new(*number)));
        
        if let Some(ref mut value) = previous {
            value.next = aux;
        }
        previous = match previous {
            Some(ref mut value) => &mut value.next,
            None => panic!("shit happens...")
        };
    }
    
    if let Some(ref mut last) = previous {
        last.next = None;        
    }
    
    head
}

pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

    let mut l1_current = l1;
    let mut l2_current = l2;

    let mut previous_sum_overflow = 0;

    let mut head: Option<Box<ListNode>> = None;
    let mut previous_node: &mut Option<Box<ListNode>> = &mut None;

    while l1_current.is_some() || l2_current.is_some() {
        let first_value = match l1_current {
            Some(ref a) => a.val,
            None => 0
        };
        let second_value = match l2_current {
            Some(ref a) => a.val,
            None => 0
        }; // ! do a treatment to know if the values exist

        let sum_of_values = first_value + second_value + previous_sum_overflow;

        let result =
            if sum_of_values >= 10 {
                previous_sum_overflow = (sum_of_values - (sum_of_values % 10))/10;
                sum_of_values % 10
            } else {
                previous_sum_overflow = 0;
                sum_of_values
            };

        let new_node = Some(Box::new(ListNode::new(result)));

        if let None = previous_node {
            head = new_node;
            previous_node = &mut head;
        } else if let Some(val) = previous_node {
            val.next = new_node;

            previous_node = &mut val.next;
        }

        l1_current = match l1_current {
            Some(val) => val.next,
            None => None
        };

        l2_current = match l2_current {
            Some(val) => val.next,
            None => None
        };        
    }

    if previous_sum_overflow > 0 {
        if let Some(val) = previous_node {
            val.next = Some(Box::new(ListNode::new(previous_sum_overflow)));
        }
    }

    head
}
