
#[cfg(test)]
mod test {
    use super::merge;

    #[test]
    fn test() {
        let mut nums1 = vec![1,2,3,0,0,0];
        let m = 3;
        let mut nums2 = vec![2,5,6];
        let n = 3;
        merge(&mut nums1, m, &mut nums2, n);
    }
}

pub fn merge(nums1: &mut Vec<i32>, m: usize, nums2: &mut Vec<i32>, n: usize) {
    let mut v1: Vec<i32> = Vec::with_capacity(m + n);
    v1.append(nums1);
    println!("{:?}",v1);
    v1.append(nums2);
    println!("{:?}",v1);

    v1.sort();
    println!("{:?}",v1);
}
