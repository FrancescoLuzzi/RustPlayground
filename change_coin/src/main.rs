// problems:
// given a number **VAL** and an array of numbers **COINS** return
// the total number of possible combination of numbers of **COINS** that add up to **VAL**
// https://www.youtube.com/watch?v=Mjy4hd2xgrs

use std::cmp::Reverse;

#[cfg(feature = "dp")]
use std::mem::swap;

#[cfg(feature = "dp")]
fn reset_array(arr: &mut [usize]) {
    for el in arr.iter_mut() {
        *el = 0
    }
    arr[0] = 1;
}

#[cfg(feature = "dp")]
fn get_num_combinations(coins: &mut [usize], value: usize) -> usize {
    coins.sort_by_key(|&num| Reverse(num));
    let mut curr: Vec<usize> = vec![0; value + 1];
    let mut prev: Vec<usize> = vec![0; value + 1];
    prev[0] = 1;
    for coin in coins.iter() {
        reset_array(&mut curr);
        for i in *coin..=value {
            if let Some(count) = curr.get_mut(i - coin) {
                curr[i] = prev[i] + *count;
            }
        }
        swap(&mut curr, &mut prev);
    }
    *prev.last().unwrap()
}

#[cfg(feature = "naive")]
fn recurse(coins: &[usize], value: usize) -> usize {
    if value == 0 {
        return 1;
    }
    if coins.is_empty() {
        return 0;
    }
    recurse(&coins[1..coins.len()], value)
        + if value >= coins[0] {
            recurse(coins, value - coins[0])
        } else {
            0
        }
}

#[cfg(feature = "naive")]
fn get_num_combinations(coins: &mut [usize], value: usize) -> usize {
    coins.sort_by_key(|&num| Reverse(num));
    recurse(coins, value)
}

fn main() {
    println!("{}", get_num_combinations(&mut [1, 2], 5));
}

#[cfg(test)]
mod test {
    use crate::get_num_combinations;
    use rstest::rstest;

    #[rstest]
    #[case([1,2,3].to_vec(),5,5)]
    #[case([1,2].to_vec(),5,3)]
    #[case([1,2,3].to_vec(),7,8)]
    fn test_get_combinations(
        #[case] mut coins: Vec<usize>,
        #[case] value: usize,
        #[case] expected: usize,
    ) {
        assert_eq!(get_num_combinations(&mut coins, value), expected);
    }
}
