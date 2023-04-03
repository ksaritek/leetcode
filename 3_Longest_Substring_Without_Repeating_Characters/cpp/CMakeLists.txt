cmake_minimum_required(VERSION 3.0.0)
project(solution VERSION 0.1.0)

# Set the C++ standard to C++20
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)



# Add the executable for the main program
add_executable(solution main.cpp)
# Add the executable for the test program
add_executable(tests test.cpp)

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)

include(CTest)

include_directories($ENV{HOME}/vcpkg/installed/x64-osx/include)
find_package(GTest CONFIG REQUIRED)
target_link_libraries(tests PRIVATE GTest::gtest GTest::gtest_main GTest::gmock GTest::gmock_main)

enable_testing()
# Add the test case
add_test(NAME Test COMMAND tests)
