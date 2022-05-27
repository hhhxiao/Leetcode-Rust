use std::cmp::min;
use std::collections::btree_set::BTreeSet;
use std::vec;

pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
    let mut arr: [Vec<i32>; 3] = [vec![], vec![], vec![]];
    let mut sum = 0;
    for i in nums {
        let idx = (i % 3) as usize;
        arr[idx].push(i);
        sum += i;
    }
    arr[0].sort();
    arr[1].sort();
    arr[2].sort();
    println!("{}", sum);
    let mut sub = i32::MAX;
    match sum % 3 {
        1 => {
            let mut v1 = i32::MAX;
            let mut v2 = i32::MAX;
            if !arr[1].is_empty() {
                v1 = *arr[1].first().unwrap();
            }

            if arr[2].len() >= 2 {
                let mut iter = arr[2].iter();
                let x = *iter.next().unwrap();
                let y = *iter.next().unwrap();
                v2 = x + y;
            }
            sub = min(v1, v2);
        }

        2 => {
            let mut v1 = i32::MAX;
            let mut v2 = i32::MAX;
            if !arr[2].is_empty() {
                v1 = *arr[2].first().unwrap();
            }

            if arr[1].len() >= 2 {
                let mut iter = arr[1].iter();
                let x = *iter.next().unwrap();
                let y = *iter.next().unwrap();
                v2 = x + y;
            }
            println!("2: {}  {}", v1, v2);
            sub = min(v1, v2);
        }

        _ => {
            sub = 0;
        }
    }
    if sub == i32::MAX {
        return 0;
    }
    sum - sub
}