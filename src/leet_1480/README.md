# 1480. Running Sum of 1D Array

Given an array `nums`. We define a running sum of an array as `runningSum[i] = sum(nums[0]...nums[i])`.

Constraints:

- The number of nodes in the list is in the range `[1, 1000]`.
- The value of each element in the array is in the range `[-10^6, 10^6]`

### Solution 1 - Memoizing the running sum

```sh
Input : [i32;5] = [3, 1, 2, 10, 1]
Output : Vec<i32> = [3, 4, 6, 16, 17]
Running Sum  = 17
```

_Time Complexity = O(N)_
We perform the same number of operations on each element in the array.

_Space Complexity = O(1)_
The output array's space is constant. Aside from the inpiut and ouput arrays, the space is constant.

### Solution 2 - Modifying the input array

```sh
Input : [i32;5] = [3, 1, 2, 10, 1]
Output : Vec<i32> = [3, 4, 6, 16, 17]
Running Sum  = 17
```

_Time Complexity = O(N)_
We perform the same number of operations on each element in the array.

_Space Complexity = O(1)_
We only store the input array.
