#ifndef SOLUTION_H
#define SOLUTION_H

// ListNode struct
struct ListNode
{
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(nullptr) {}
};

// Solution class
class Solution
{
public:
    ListNode *addTwoNumbers(ListNode *l1, ListNode *l2);
};

#endif
