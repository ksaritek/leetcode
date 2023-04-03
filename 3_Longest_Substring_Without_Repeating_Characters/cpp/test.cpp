#include <gtest/gtest.h>
#include "main.cpp"

TEST(lengthOfLongestSubstring, Example1)
{
    std::string s = "abcabcbb";
    int expected_output = 3;
    ASSERT_EQ(lengthOfLongestSubstring(s), expected_output);
}

TEST(lengthOfLongestSubstring, Example2)
{
    std::string s = "bbbbb";
    int expected_output = 1;
    ASSERT_EQ(lengthOfLongestSubstring(s), expected_output);
}

TEST(lengthOfLongestSubstring, Example3)
{
    std::string s = "pwwkew";
    int expected_output = 3;
    ASSERT_EQ(lengthOfLongestSubstring(s), expected_output);
}
