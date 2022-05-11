// Copyright 2022 tison <wander4096@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub fn bubble_sort_1a<T: PartialOrd, const N: usize>(ns: &mut [T; N]) {
    let mut sorted = false;
    let mut n = N;
    while !sorted {
        sorted = true;
        for i in 1..n {
            if ns[i - 1] > ns[i] {
                ns.swap(i - 1, i);
                sorted = false;
            }
        }
        n -= 1;
    }
}

pub fn count_ones(mut n: u32) -> u32 {
    let mut ones = 0;
    while 0 < n {
        ones += 1 & n;
        n >>= 1;
    }
    ones
}

#[cfg(test)]
mod tests {
    fn is_sorted<T: PartialOrd, const N: usize>(ns: &[T; N]) -> bool {
        for i in 1..N {
            if ns[i - 1] > ns[i] {
                return false;
            }
        }
        true
    }

    #[test]
    fn bubble_sort_1a() {
        let mut ns = [6, 5, 4, 3, 2, 1];
        super::bubble_sort_1a(&mut ns);
        assert!(is_sorted(&ns), "unsorted: {:?}", ns);
    }

    #[test]
    fn count_ones() {
        assert_eq!(0, super::count_ones(0));
        assert_eq!(1, super::count_ones(1));
        assert_eq!(1, super::count_ones(2));
        assert_eq!(6, super::count_ones(441));
    }
}
