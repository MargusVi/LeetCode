# [20. Valid Parentheses](https://leetcode.com/problems/valid-parentheses/)

**(Easy)**

Given a string `s` containing just the characters `'('`, `')'`, `'{'`, `'}'`, `'['` and `']'`, determine if the input string is valid.

An input string is valid if:

1. Open brackets must be closed by the same type of brackets.
2. Open brackets must be closed in the correct order.
3. Every close bracket has a corresponding open bracket of the same type.

## **Example 1:**

<pre><strong>Input:</strong> s = "()"
<strong>Output:</strong> true
</pre>

## **Example 2:**

<pre><strong>Input:</strong> s = "()[]{}"
<strong>Output:</strong> true
</pre>

## **Example 3:**

<pre><strong>Input:</strong> s = "(]"
<strong>Output:</strong> false
</pre>

## **Example 4:**

<pre><strong>Input:</strong> s = "([])"
<strong>Output:</strong> true
</pre>

## **Constraints:**

* `1 <= s.length <= 10<sup>4</sup>`
* `s` consists of parentheses only `'()[]{}'`.
