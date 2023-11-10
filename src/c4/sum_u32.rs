fn sum_u32(xx: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for i in xx {
        if let Some(v) = sum.checked_add(*i) {
            sum = v;
        } else {
            return None;
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use crate::c4::sum_u32::sum_u32;

    #[test]
    fn test_sum_u32() {
        let cases = [
            ([1, 2, 3], Some(6)),
            ([1, 1, (u32::MAX - 2)], Some(u32::MAX)),
            ([1, 2, (u32::MAX - 2)], None),
        ];
        for (input, res) in cases {
            assert_eq!(sum_u32(&input), res);
        }
    }
}