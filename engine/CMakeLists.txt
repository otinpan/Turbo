cmake_minimum_required(VERSION 3.28)

project(Engine VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

find_package(Vulkan REQUIRED)

find_package(glfw3 CONFIG QUIET)
if(NOT glfw3_FOUND)
  set(GLFW_BUILD_EXAMPLES OFF CACHE BOOL "" FORCE)
  set(GLFW_BUILD_TESTS OFF CACHE BOOL "" FORCE)
  set(GLFW_BUILD_DOCS OFF CACHE BOOL "" FORCE)
  add_subdirectory("D:/MyGame/glfw-3.4" "${CMAKE_BINARY_DIR}/_deps/glfw")
endif()

find_package(glm CONFIG QUIET)
if(NOT glm_FOUND)
  set(GLM_BUILD_TESTS OFF CACHE BOOL "" FORCE)
  set(GLM_BUILD_INSTALL OFF CACHE BOOL "" FORCE)
  add_subdirectory("D:/MyGame/glm-1.0.1" "${CMAKE_BINARY_DIR}/_deps/glm")
endif()

find_program(SLANGC_EXECUTABLE
  NAMES slangc slangc.exe
  HINTS
    "$ENV{VULKAN_SDK}/Bin"
    "$ENV{VULKAN_SDK}/bin"
    "C:/VulkanSDK/1.4.341.1/Bin"
    "C:/VulkanSDK/1.4.341.1/bin"
  REQUIRED
)

set(SHADER_SOURCE "${CMAKE_CURRENT_SOURCE_DIR}/src/shaders/shader.slang")
set(SHADER_OUTPUT_DIR "${CMAKE_CURRENT_BINARY_DIR}/shaders")
set(VERT_SHADER "${SHADER_OUTPUT_DIR}/vert.spv")
set(FRAG_SHADER "${SHADER_OUTPUT_DIR}/frag.spv")

add_custom_command(
  OUTPUT "${VERT_SHADER}"
  COMMAND "${CMAKE_COMMAND}" -E make_directory "${SHADER_OUTPUT_DIR}"
  COMMAND "${SLANGC_EXECUTABLE}"
    "${SHADER_SOURCE}"
    -target spirv
    -profile spirv_1_4
    -emit-spirv-directly
    -fvk-use-entrypoint-name
    -entry vertMain
    -stage vertex
    -o "${VERT_SHADER}"
  # if the shader source file changes, we need to recompile the shader
  DEPENDS "${SHADER_SOURCE}"
  COMMENT "Compiling vertex shader"
  VERBATIM
)

add_custom_command(
  OUTPUT "${FRAG_SHADER}"
  COMMAND "${CMAKE_COMMAND}" -E make_directory "${SHADER_OUTPUT_DIR}"
  COMMAND "${SLANGC_EXECUTABLE}"
    "${SHADER_SOURCE}"
    -target spirv
    -profile spirv_1_4
    -emit-spirv-directly
    -fvk-use-entrypoint-name
    -entry fragMain
    -stage fragment
    -o "${FRAG_SHADER}"
  DEPENDS "${SHADER_SOURCE}"
  COMMENT "Compiling fragment shader"
  VERBATIM
)

# Create a custom target that depends on the compiled shader files
add_custom_target(shaders DEPENDS
  "${VERT_SHADER}"
  "${FRAG_SHADER}"
)

add_executable(engine
  src/main.cpp
)

target_compile_definitions(engine PRIVATE
  VULKAN_HPP_DISPATCH_LOADER_DYNAMIC=1
  VULKAN_HPP_NO_STRUCT_CONSTRUCTORS=1
)

# Link against the Vulkan C++ bindings, GLFW, and GLM
target_link_libraries(engine PRIVATE
  Vulkan::Vulkan
  glfw
  glm::glm
)

add_dependencies(engine shaders)
