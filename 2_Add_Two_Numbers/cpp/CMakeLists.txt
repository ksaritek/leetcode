cmake_minimum_required(VERSION 3.0)

# Set the project name and version
project(addTwoNumbers VERSION 1.0)

# Set the C++ standard to C++20
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Add the executable for the main program
add_executable(main main.cpp solution.cpp)

# Add the executable for the test program
add_executable(tests test.cpp solution.cpp)

include_directories($ENV{HOME}/git/personal/cpp/vcpkg/installed/x64-osx/include)
find_package(GTest CONFIG REQUIRED)
target_link_libraries(tests PRIVATE GTest::gtest GTest::gtest_main GTest::gmock GTest::gmock_main)

# Enable testing
enable_testing()

# Add the test case
add_test(NAME AddTwoNumbersTest COMMAND tests)
