cmake_minimum_required(VERSION 3.28)

project(Engine VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

find_package(Vulkan REQUIRED)
find_package(glfw3 CONFIG REQUIRED)

add_executable(engine
  src/main.cpp
)

target_compile_definitions(engine PRIVATE
  VULKAN_HPP_DISPATCH_LOADER_DYNAMIC=1
  VULKAN_HPP_NO_STRUCT_CONSTRUCTORS=1
)

target_link_libraries(engine PRIVATE Vulkan::Vulkan)
target_link_libraries(engine PRIVATE glfw)
