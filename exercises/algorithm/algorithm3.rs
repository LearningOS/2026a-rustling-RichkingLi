/*
	排序
	本题要求你实现一个排序算法
	你可以使用冒泡排序、插入排序、堆排序等
*/

fn sort<T: Ord>(array: &mut [T]) {
	let n = array.len();
	for i in 0..n {
		// 每轮把当前最大的元素“冒泡”到末尾；用 saturating_sub 防止 n=0 时下溢
		let last = n.saturating_sub(1).saturating_sub(i);
		for j in 0..last {
			if array[j] > array[j + 1] {
				array.swap(j, j + 1);
			}
		}
	}
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
