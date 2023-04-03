#include <iostream>
#include <unordered_set>
#include <algorithm>

int main(int, char **)
{
    std::cout << "Hello, world!\n";
}

int lengthOfLongestSubstring(std::string s)
{
    int n = s.length();
    std::unordered_set<char> hash_set;
    int left = 0, right = 0, max_len = 0;

    while (right < n)
    {
        if (hash_set.find(s[right]) == hash_set.end())
        {
            hash_set.insert(s[right]);
            max_len = std::max(max_len, int(hash_set.size()));
            right++;
        }
        else
        {
            hash_set.erase(s[left]);
            left++;
        }
    }

    return max_len;
}
