# [58. Length of Last Word](https://leetcode.com/problems/length-of-last-word/)

**(Easy)**

Given a string `s` consisting of words and spaces, return *the length of the **last** word in the string.*

A **word** is a maximal substring consisting of non-space characters only.

## **Example 1:**

<pre><strong>Input:</strong> s = "Hello World"
<strong>Output:</strong> 5
<strong>Explanation:</strong> The last word is "World" with length 5.
</pre>

## **Example 2:**

<pre><strong>Input:</strong> s = "   fly me   to   the moon  "
<strong>Output:</strong> 4
<strong>Explanation:</strong> The last word is "moon" with length 4.
</pre>

## **Example 3:**

<pre><strong>Input:</strong> s = "luffy is still joyboy"
<strong>Output:</strong> 6
<strong>Explanation:</strong> The last word is "joyboy" with length 6.
</pre>

## **Constraints:**

* `1 <= s.length <= 10<sup>4</sup>`
* `s` consists of only English letters and spaces `' '`.
* There will be at least one word in `s`.
