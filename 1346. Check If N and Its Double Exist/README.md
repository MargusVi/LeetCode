# [1346. Check If N and Its Double Exist](https://leetcode.com/problems/check-if-n-and-its-double-exist/)

**(Easy)**

Given an array `arr` of integers, check if there exist two indices `i` and `j` such that :

* `i != j`
* `0 <= i, j < arr.length`
* `arr[i] == 2 * arr[j]`

## **Example 1:**

<pre><strong>Input:</strong> arr = [10,2,5,3]
<strong>Output:</strong> true
<strong>Explanation:</strong> For i = 0 and j = 2, arr[i] == 10 == 2 * 5 == 2 * arr[j]
</pre>

## **Example 2:**

<pre><strong>Input:</strong> arr = [3,1,7,11]
<strong>Output:</strong> false
<strong>Explanation:</strong> There is no i and j that satisfy the conditions.
</pre>

## **Constraints:**

* `2 <= arr.length <= 500`
* `-10<sup>3</sup> <= arr[i] <= 10<sup>3</sup>`
