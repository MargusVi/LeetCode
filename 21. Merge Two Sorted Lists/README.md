# [21. Merge Two Sorted Lists](https://leetcode.com/problems/merge-two-sorted-lists/)

**(Easy)**

You are given the heads of two sorted linked lists `list1` and `list2`.

Merge the two lists into one **sorted** list. The list should be made by splicing together the nodes of the first two lists.

Return  *the head of the merged linked list* .

## **Example 1:**

![](https://assets.leetcode.com/uploads/2020/10/03/merge_ex1.jpg)

<pre><strong>Input:</strong> list1 = [1,2,4], list2 = [1,3,4]
<strong>Output:</strong> [1,1,2,3,4,4]
</pre>

## **Example 2:**

<pre><strong>Input:</strong> list1 = [], list2 = []
<strong>Output:</strong> []
</pre>

## **Example 3:**

<pre><strong>Input:</strong> list1 = [], list2 = [0]
<strong>Output:</strong> [0]
</pre>

## **Constraints:**

* The number of nodes in both lists is in the range `[0, 50]`.
* `-100 <= Node.val <= 100`
* Both `list1` and `list2` are sorted in **non-decreasing** order.
