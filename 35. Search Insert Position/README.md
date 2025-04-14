# [35. Search Insert Position](https://leetcode.com/problems/search-insert-position/)

**(Easy)**

Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

You must write an algorithm with `O(log n)` runtime complexity.

## **Example 1:**

<pre><strong>Input:</strong> nums = [1,3,5,6], target = 5
<strong>Output:</strong> 2
</pre>

## **Example 2:**

<pre><strong>Input:</strong> nums = [1,3,5,6], target = 2
<strong>Output:</strong> 1
</pre>

## **Example 3:**

<pre><strong>Input:</strong> nums = [1,3,5,6], target = 7
<strong>Output:</strong> 4
</pre>

## **Constraints:**

* `1 <= nums.length <= 10<sup>4</sup>`
* `-10<sup>4</sup> <= nums[i] <= 10<sup>4</sup>`
* `nums` contains **distinct** values sorted in **ascending** order.
* `-10<sup>4</sup> <= target <= 10<sup>4</sup>`
