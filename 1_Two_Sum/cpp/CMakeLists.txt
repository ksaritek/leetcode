cmake_minimum_required(VERSION 3.16)
project(TwoSum)

set(CMAKE_CXX_STANDARD 14)

add_executable(TwoSum main.cpp TwoSum.cpp)

enable_testing()

add_test(NAME Test1 COMMAND TwoSum)
set_tests_properties(Test1 PROPERTIES PASS_REGULAR_EXPRESSION "Test case 1: Passed")

add_test(NAME Test2 COMMAND TwoSum)
set_tests_properties(Test2 PROPERTIES PASS_REGULAR_EXPRESSION "Test case 2: Passed")

add_test(NAME Test3 COMMAND TwoSum)
set_tests_properties(Test3 PROPERTIES PASS_REGULAR_EXPRESSION "Test case 3: Passed")
