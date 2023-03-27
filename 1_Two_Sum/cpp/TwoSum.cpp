#include "TwoSum.h"
#include <unordered_map>

// This approach has a time complexity of O(n) since each lookup and insertion in the map takes constant time on average
std::vector<int> TwoSum::twoSum(std::vector<int> &nums, int target)
{
    std::vector<int> result;
    std::unordered_map<int, int> numMap;

    for (int i = 0; i < nums.size(); i++)
    {
        int complement = target - nums[i];
        if (numMap.find(complement) != numMap.end())
        {
            result.push_back(numMap[complement]);
            result.push_back(i);
            return result;
        }
        numMap[nums[i]] = i;
    }

    return result;
}
