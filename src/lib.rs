extern crate core;

pub mod sort {
    pub fn bubble_sort<T: PartialOrd>(vx: &mut Vec<T>) {
        let n = vx.len();
        for i in 0..n {
            for j in 0..(n - i - 1) {
                if vx[j] > vx[j + 1] {
                    vx.swap(j, j + 1)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort;

    #[test]
    fn test_bubble_sort_empty() {
        let mut input: Vec<isize> = vec![];
        sort::bubble_sort(&mut input);
        assert_eq!(input, vec![]);
    }

    #[test]
    fn test_bubble_sort_isize() {
        let mut cases = vec![
            (vec![1], vec![1]),
            (vec![1, 2], vec![1, 2]),
            (vec![1, 2, 3], vec![1, 2, 3]),
            (vec![3, 2, 1], vec![1, 2, 3]),
            (vec![2, 3, 1], vec![1, 2, 3]),
        ];

        for (input, output) in cases.iter_mut() {
            sort::bubble_sort(input);
            assert_eq!(input, output)
        }
    }

    #[test]
    fn test_bubble_sort_f64() {
        let mut cases = vec![
            (vec![1.0], vec![1.0]),
            (vec![1.1, 2.1], vec![1.1, 2.1]),
            (vec![1.1, 2.2, 3.3], vec![1.1, 2.2, 3.3]),
            (vec![3.1, 2.2, 1.3], vec![1.3, 2.2, 3.1]),
            (vec![2.1, 3.2, 1.3], vec![1.3, 2.1, 3.2]),
        ];

        for (input, output) in cases.iter_mut() {
            sort::bubble_sort(input);
            assert_eq!(input, output)
        }
    }
}