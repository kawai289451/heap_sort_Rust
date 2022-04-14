// 【課題4】 ヒープソート（二分ヒープ）
// ヒープソート（二分ヒープ）のアルゴリズムを Rust で実装してください。
//
// 仕様
// 0 以上 100 以下のランダムな整数を 15 回発生させ、その整数を降順に並び替える。
//
// 発展課題（任意）
// 並び替えのたびに木構造を標準出力する。
//
// 提出期限
// 2022/04/15（金）12:00 まで

use rand::Rng;

fn main() {
    println!("--- Heap Sort! ---");

    const MIN: i32 = 0;
    const MAX: i32 = 100;
    const NUM: usize = 15;
    let rand_vec: Vec<i32> = gen_rand_vec(MIN, MAX, NUM);
    println!("original array: {:?}", rand_vec);

    let heap_sort_arr: Vec<i32> = heap_sort(&rand_vec);
    println!("sorted array: {:?}", heap_sort_arr);
}

/**
 * ランダムな整数を発生させ、ベクタ配列として取得します。
 * 引数で整数の範囲、回数を指定します。
 * 
 * # Arguments
 * * `min` - 範囲の下限
 * * `max` - 範囲の上限
 * * `num` - 回数
 */
fn gen_rand_vec (min: i32, max: i32, num: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..(num + 1)).map(|_| rng.gen_range(min, max)).collect()
}

/**
 * ベクタ配列をヒープソートします。
 * 
 * # Arguments
 * * `arr` - ベクタ配列
 */
fn heap_sort (arr: &Vec<i32>) -> Vec<i32> {
    let mut a: Vec<i32> = arr.clone();
    let mut l: usize = a.len();

    let max_val: usize = (l as f64 / 2.).floor() as usize;
    for i in (0..=max_val).rev() { heapify(&mut a, i, l); }
    let max_val: usize = a.len() - 1;
    for i in (1..=max_val).rev() {
        a.swap(0, i);
        l -= 1;
        heapify(&mut a, 0, l);
    }
    a
}

fn heapify (a: &mut Vec<i32>, i: usize, l: usize) -> Vec<i32> {
    let left: usize = 2 * i + 1;
    let right: usize = 2 * i + 2;
    let mut max: usize = i;
    if left < l && a[left] > a[max] { max = left }
    if right < l && a[right] > a[max] { max = right }
    if max != i {
        println!("> sort");
        println!{"current: {:?}", a};
        if right < l {
            println!("before: {} - [{}] - {}  (idx: {}, {}, {})", a[left], a[i], a[right], left, i, right);
        }
        a.swap(max, i);
        if right < l {
            println!("after:  {} - [{}] - {}\n", a[left], a[i], a[right]);
        }
        heapify(a, max, l);
    }
    a.to_vec()
}

// https://www.30secondsofcode.org/js/s/heapsort
//
// const heapsort = arr => {
//     const a = [...arr];
//     let l = a.length;
//
//     const heapify = (a, i) => {
//       const left = 2 * i + 1;
//       const right = 2 * i + 2;
//       let max = i;
//       if (left < l && a[left] > a[max]) max = left;
//       if (right < l && a[right] > a[max]) max = right;
//       if (max !== i) {
//         [a[max], a[i]] = [a[i], a[max]];
//         heapify(a, max);
//       }
//     };
//
//     for (let i = Math.floor(l / 2); i >= 0; i -= 1) heapify(a, i);
//     for (i = a.length - 1; i > 0; i--) {
//       [a[0], a[i]] = [a[i], a[0]];
//       l--;
//       heapify(a, 0);
//     }
//     return a;
//   };
// Examples
// heapsort([6, 3, 4, 1]); // [1, 3, 4, 6]


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heap_sorted () {
        let test_arr = vec![6, 3, 4, 1];
        let exp_arr = vec![1, 3, 4, 6];
        match heap_sort(&test_arr) {
            Ok(result) => assert_eq!(result, exp_arr),
            Err(_) => assert!(false, "failed!")
        }
    }

    #[test]
    fn smaller_than_0 () {
        let test_arr = vec![6, 3, 4 , 1, -1];
        let exp_msg = "Minimum value is 0!";
        match heap_sort(&test_arr) {
            Ok(_) => assert!(false, "Should not return Ok!"),
            Err(msg) => assert_eq!(msg, exp_msg)
        }
    }

    #[test]
    fn bigger_than_100 () {
        let test_arr = vec![6, 3, 4 , 1, 101];
        let exp_msg = "Maximum value is 100!";
        match heap_sort(&test_arr) {
            Ok(_) => assert!(false, "Should not return Ok!"),
            Err(msg) => assert_eq!(msg, exp_msg)
        }
    }
}