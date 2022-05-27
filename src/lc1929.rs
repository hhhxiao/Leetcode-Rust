 pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {

    let cl =  nums.clone();
     let mut v :Vec<i32> = Vec::new();
    for i in nums{
        v.push(i.clone())
    }
    for i in cl{
        v.push(i)
    }
     v
     //nums.repeat(2)
    }
