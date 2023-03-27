#include <iostream>
#include "TwoSum.h"

int main()
{
    TwoSum solution;

    // Test case 1
    std::vector<int> nums1{2, 7, 11, 15};
    int target1 = 9;
    std::vector<int> result1 = solution.twoSum(nums1, target1);
    std::cout << "Test case 1: ";
    if (result1.size() == 2 && result1[0] == 0 && result1[1] == 1)
    {
        std::cout << "Passed\n";
    }
    else
    {
        std::cout << "Failed\n";
    }

    // Test case 2
    std::vector<int> nums2{3, 2, 4};
    int target2 = 6;
    std::vector<int> result2 = solution.twoSum(nums2, target2);
    std::cout << "Test case 2: ";
    if (result2.size() == 2 && result2[0] == 1 && result2[1] == 2)
    {
        std::cout << "Passed\n";
    }
    else
    {
        std::cout << "Failed\n";
    }

    // Test case 3
    std::vector<int> nums3{3, 3};
    int target3 = 6;
    std::vector<int> result3 = solution.twoSum(nums3, target3);
    std::cout << "Test case 3: ";
    if (result3.size() == 2 && result3[0] == 0 && result3[1] == 1)
    {
        std::cout << "Passed\n";
    }
    else
    {
        std::cout << "Failed\n";
    }

    return 0;
}
