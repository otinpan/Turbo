cmake_minimum_required(VERSION 3.28)

project(Engine VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

find_package(Vulkan REQUIRED)

set(TURBO_GLFW_SOURCE_DIR "D:/MyGame/glfw-3.4" CACHE PATH "GLFW source directory")
if(EXISTS "${TURBO_GLFW_SOURCE_DIR}/CMakeLists.txt")
  set(GLFW_BUILD_EXAMPLES OFF CACHE BOOL "" FORCE)
  set(GLFW_BUILD_TESTS OFF CACHE BOOL "" FORCE)
  set(GLFW_BUILD_DOCS OFF CACHE BOOL "" FORCE)
  add_subdirectory("${TURBO_GLFW_SOURCE_DIR}" "${CMAKE_BINARY_DIR}/_deps/glfw")
else()
  find_package(glfw3 CONFIG REQUIRED)
endif()

set(TURBO_GLM_SOURCE_DIR "D:/MyGame/glm-1.0.1" CACHE PATH "GLM source directory")
if(EXISTS "${TURBO_GLM_SOURCE_DIR}/CMakeLists.txt")
  set(GLM_BUILD_TESTS OFF CACHE BOOL "" FORCE)
  set(GLM_BUILD_INSTALL OFF CACHE BOOL "" FORCE)
  add_subdirectory("${TURBO_GLM_SOURCE_DIR}" "${CMAKE_BINARY_DIR}/_deps/glm")
else()
  find_package(glm CONFIG REQUIRED)
endif()

add_executable(engine
  src/main.cpp
)

target_compile_definitions(engine PRIVATE
  VULKAN_HPP_DISPATCH_LOADER_DYNAMIC=1
  VULKAN_HPP_NO_STRUCT_CONSTRUCTORS=1
)

target_link_libraries(engine PRIVATE Vulkan::Vulkan)
target_link_libraries(engine PRIVATE glfw)
target_link_libraries(engine PRIVATE glm::glm)
