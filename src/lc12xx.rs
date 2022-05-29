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

pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
    /**
    (x1,y1) (x2,y2) (x3,y3)
    (y2 - y1)/(x2 - x1) == (y3 - y2)/(x3 - x2)
    (y2 - y1)(x3 - x2) == (y3 - y2)(x2 - x1)
     */
    for i in 0..coordinates.len() - 2 {
        let v1 = &coordinates[i];
        let v2 = &coordinates[i + 1];
        let v3 = &coordinates[i + 2];
        let left = (v2[1] - v1[1]) * (v3[0] - v2[0]);
        let right = (v3[1] - v2[1]) * (v2[0] - v1[0]);
        if left != right {
            return false
        }
    }

    true
}