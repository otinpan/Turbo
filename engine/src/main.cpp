#if defined(__INTELLISENSE__) || !defined(USE_CPP20_MODULES)
#include <vulkan/vulkan_raii.hpp>
#else
import vulkan_hpp;
#endif
#include <GLFW/glfw3.h>

#include <cstdlib>
#include <iostream>
#include <stdexcept>

const uint32_t WIDTH = 800;
const uint32_t HEIGHT = 600;

class HelloTriangleApplication {
 public:
  ~HelloTriangleApplication() {
    cleanup();
  }

  void run() {
    try {
      initWindow();
      initVulkan();
      mainLoop();
    } catch (...) {
      cleanup();
      throw;
    }
  }

 private:
  GLFWwindow* window = nullptr;
  bool glfw_initialized = false;

  void initWindow() {
    if (glfwInit() != GLFW_TRUE) {
      throw std::runtime_error("failed to initialize GLFW");
    }
    glfw_initialized = true;

    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
    glfwWindowHint(GLFW_RESIZABLE, GLFW_FALSE);

    window = glfwCreateWindow(WIDTH, HEIGHT, "Vulkan", nullptr, nullptr);
    if (window == nullptr) {
      throw std::runtime_error("failed to create GLFW window");
    }
  }

  void initVulkan() {
    std::cout << "Hello Vulkan!" << std::endl;
  }

  void mainLoop() {
    while (!glfwWindowShouldClose(window)) {
      glfwPollEvents();
    }
  }

  void cleanup() {
    if (window != nullptr) {
      glfwDestroyWindow(window);
      window = nullptr;
    }

    if (glfw_initialized) {
      glfwTerminate();
      glfw_initialized = false;
    }
  }
};


int main() {
  try {
    HelloTriangleApplication app;
    app.run();
  } catch (const std::exception& e) {
    std::cerr << e.what() << std::endl;
    return EXIT_FAILURE;
  }

  return EXIT_SUCCESS;
}
