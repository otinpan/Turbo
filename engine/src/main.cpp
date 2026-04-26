#include <algorithm>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <memory>
#include <stdexcept>
#include <vector>
#include <vulkan/vulkan.hpp>

#if defined(__INTELLISENSE__) || !defined(USE_CPP20_MODULES)
#include <vulkan/vulkan_raii.hpp>
#else
import vulkan_hpp;
#endif
#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

const uint32_t WIDTH = 800;
const uint32_t HEIGHT = 600;

const std::vector<char const *> validationLayers={
  "VK_LAYER_KHRONOS_validation"
};

#ifdef NDEBUG
constexpr bool enableValidationLayers=false;
#else
constexpr bool enableValidationLayers=true;
#endif

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

  vk::raii::Context context;
  vk::raii::Instance instance=nullptr;
  vk::raii::DebugUtilsMessengerEXT debugMessenger=nullptr;

  vk::raii::PhysicalDevice physicalDevice=nullptr;
  std::vector<const char*> requiredDeviceExtension={vk::KHRSwapchainExtensionName};

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
    createInstance();
    setupDebugMessenger();
    pickPhysicalDevice();
    printPhysicalDevices();
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


  void createInstance()
  {
      constexpr vk::ApplicationInfo appInfo{.pApplicationName   = "Hello Triangle",
                                            .applicationVersion = VK_MAKE_VERSION(1, 0, 0),
                                            .pEngineName        = "No Engine",
                                            .engineVersion      = VK_MAKE_VERSION(1, 0, 0),
                                            .apiVersion         = vk::ApiVersion13
      };

      // Get the required layers
      std::vector<char const*> requiredLayers;
      if (enableValidationLayers){
        requiredLayers.assign(validationLayers.begin(),validationLayers.end());
      }

      // Check if the required layers are supported by the Vulkan implementation
      auto layerProperties=context.enumerateInstanceLayerProperties();
      auto unsupportedLayerIt = std::ranges::find_if(
          requiredLayers,
          [&layerProperties](auto const& requiredLayer) {
              return std::ranges::none_of(
                  layerProperties,
                  [requiredLayer](auto const& layerProperty) {
                      return strcmp(layerProperty.layerName, requiredLayer) == 0;
                  }
              );
          }
      );

      if(unsupportedLayerIt!=requiredLayers.end()){
        throw std::runtime_error("Required layer not supported: "+std::string(*unsupportedLayerIt));
      }
      
      // Get required extensions
      auto requiredExtensions=getRequiredInstanceExtensions();
      // Check if the required extensions are supported by the Vulkan implementation
      auto extensionProperties=context.enumerateInstanceExtensionProperties();
      auto unsupportedPropertyIt=std::ranges::find_if(
          requiredExtensions,
          [&extensionProperties](auto const& requiredExtension){
            return std::ranges::none_of(
                extensionProperties,
                [requiredExtension](auto const& extensionProperty){
                  return strcmp(extensionProperty.extensionName,requiredExtension)==0;
                }
            );
          }
      );

      if(unsupportedPropertyIt!=requiredExtensions.end()){
        throw std::runtime_error("Required extension not supported: "+std::string(*unsupportedPropertyIt));
      }

      vk::InstanceCreateInfo createInfo{
        .pApplicationInfo=&appInfo,
        .enabledLayerCount=static_cast<uint32_t>(requiredLayers.size()),
        .ppEnabledLayerNames=requiredLayers.data(),
        .enabledExtensionCount=static_cast<uint32_t>(requiredExtensions.size()),
        .ppEnabledExtensionNames=requiredExtensions.data()
      };

      instance=vk::raii::Instance(context,createInfo);
      
  }

  bool isDeviceSuitable(vk::raii::PhysicalDevice const &physicalDevice){
    // check if the physicalDevice supports the Vulkan 1.3 API version
    bool supportsVulkan1_3=physicalDevice.getProperties().apiVersion>=vk::ApiVersion13;

    // check if any of the queue families support graphics operations
    auto queueFamilies=physicalDevice.getQueueFamilyProperties();
    bool supportsGraphics=std::ranges::any_of(
        queueFamilies,
        [](auto const &qfp){return !!(qfp.queueFlags & vk::QueueFlagBits::eGraphics);}
        );

    // check if all required physicalDevice extensions are available
    auto availableDeviceExtensions=physicalDevice.enumerateDeviceExtensionProperties();
    bool supportsRequiredExtensions=
      std::ranges::all_of(
          requiredDeviceExtension,
          [&availableDeviceExtensions](auto const &requiredDeviceExtension){
            return std::ranges::any_of(
                availableDeviceExtensions,
                [requiredDeviceExtension](auto const &availableDeviceExtension){
                  return strcmp(availableDeviceExtension.extensionName,requiredDeviceExtension)==0;
                }
            ); 
          }
      );

    // Check if the physicalDevice supports the required features (dynamic rendering and extended dynamic state)
  auto features =
    physicalDevice
      .template getFeatures2<
      vk::PhysicalDeviceFeatures2,
      vk::PhysicalDeviceVulkan13Features,
      vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT>();

  bool supportsRequiredFeatures 
    = features
    .template get<vk::PhysicalDeviceVulkan13Features>().dynamicRendering &&
                                  features.template get<vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT>().extendedDynamicState;

  // Return true if the physicalDevice meets all the criteria
  return supportsVulkan1_3 && supportsGraphics && supportsRequiredExtensions && supportsRequiredFeatures;

  }

  void pickPhysicalDevice(){
    std::vector<vk::raii::PhysicalDevice> physicalDevices = instance.enumeratePhysicalDevices();
    auto const devIter =
      std::ranges::find_if( 
          physicalDevices, [&]( auto const & physicalDevice ){
            return isDeviceSuitable( physicalDevice ); 
          } 
      );

    if ( devIter == physicalDevices.end() ){
      throw std::runtime_error( "failed to find a suitable GPU!" );
    }
    physicalDevice = *devIter;
  }

  void printPhysicalDevices() {
  std::vector<vk::raii::PhysicalDevice> physicalDevices = instance.enumeratePhysicalDevices();

  std::cout << "Found " << physicalDevices.size() << " devices\n";

  for (size_t i = 0; i < physicalDevices.size(); ++i) {
    auto props = physicalDevices[i].getProperties();

    std::cout << "Device " << i << ":\n";
    std::cout << "  Name: " << props.deviceName << "\n";
    std::cout << "  API Version: " 
              << VK_VERSION_MAJOR(props.apiVersion) << "."
              << VK_VERSION_MINOR(props.apiVersion) << "."
              << VK_VERSION_PATCH(props.apiVersion) << "\n";
    std::cout << "  Vendor ID: " << props.vendorID << "\n";
    std::cout << "  Device ID: " << props.deviceID << "\n";
    std::cout << "  Type: " << vk::to_string(props.deviceType) << "\n\n";
  }
}

  void setupDebugMessenger(){
    if(!enableValidationLayers)return;

    vk::DebugUtilsMessageSeverityFlagsEXT severityFlags(
        vk::DebugUtilsMessageSeverityFlagBitsEXT::eWarning |
        vk::DebugUtilsMessageSeverityFlagBitsEXT::eError
    );

    vk::DebugUtilsMessageTypeFlagsEXT messageTypeFlags(
        vk::DebugUtilsMessageTypeFlagBitsEXT::eGeneral |
        vk::DebugUtilsMessageTypeFlagBitsEXT::ePerformance |
        vk::DebugUtilsMessageTypeFlagBitsEXT::eValidation
    );

    vk::DebugUtilsMessengerCreateInfoEXT debugUtilsMessengerCreateInfoEXT{
      .messageSeverity=severityFlags,
      .messageType=messageTypeFlags,
      .pfnUserCallback=&debugCallback
    };

    debugMessenger=instance.createDebugUtilsMessengerEXT(debugUtilsMessengerCreateInfoEXT);
  }


  std::vector<const char*> getRequiredInstanceExtensions(){
    uint32_t glfwExtensionCount=0;
    auto glfwExtensions=glfwGetRequiredInstanceExtensions(&glfwExtensionCount);

    std::vector extensions(glfwExtensions,glfwExtensions+glfwExtensionCount);
    if(enableValidationLayers){
      extensions.push_back(vk::EXTDebugUtilsExtensionName);
    }

    return extensions;
  }

  static VKAPI_ATTR VkBool32 VKAPI_CALL debugCallback(
      VkDebugUtilsMessageSeverityFlagBitsEXT severity,
      VkDebugUtilsMessageTypeFlagsEXT type,
      const VkDebugUtilsMessengerCallbackDataEXT *pCallbackData,
      void*
  ){
    if(severity & (VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT |
                   VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT)){
      std::cerr<<"validation layer: type "<<vk::to_string(vk::DebugUtilsMessageTypeFlagsEXT(type))
               <<" mst: "<<pCallbackData->pMessage<<std::endl;
    }

    return VK_FALSE;
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
