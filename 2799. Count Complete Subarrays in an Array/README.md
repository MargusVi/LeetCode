# [2799. Count Complete Subarrays in an Array](https://leetcode.com/problems/count-complete-subarrays-in-an-array/)

**(Medium)**

You are given an array `nums` consisting of **positive** integers.

We call a subarray of an array **complete** if the following condition is satisfied:

* The number of **distinct** elements in the subarray is equal to the number of distinct elements in the whole array.

Return  *the number of **complete** subarrays* .

A **subarray** is a contiguous non-empty part of an array.

## **Example 1:**

<pre><strong>Input:</strong> nums = [1,3,1,2,2]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The complete subarrays are the following: [1,3,1,2], [1,3,1,2,2], [3,1,2] and [3,1,2,2].
</pre>

## **Example 2:**

<pre><strong>Input:</strong> nums = [5,5,5,5]
<strong>Output:</strong> 10
<strong>Explanation:</strong> The array consists only of the integer 5, so any subarray is complete. The number of subarrays that we can choose is 10.
</pre>

## **Constraints:**

* `1 <= nums.length <= 1000`
* `1 <= nums[i] <= 2000`
