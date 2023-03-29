#include "gtest/gtest.h"
#include "solution.h"

TEST(addTwoNumbers, test1)
{
    ListNode *l1 = new ListNode(2);
    l1->next = new ListNode(4);
    l1->next->next = new ListNode(3);

    ListNode *l2 = new ListNode(5);
    l2->next = new ListNode(6);
    l2->next->next = new ListNode(4);

    Solution solution;
    ListNode *result = solution.addTwoNumbers(l1, l2);

    EXPECT_EQ(result->val, 7);
    result = result->next;
    EXPECT_EQ(result->val, 0);
    result = result->next;
    EXPECT_EQ(result->val, 8);
    result = result->next;
    EXPECT_EQ(result, nullptr);
}
