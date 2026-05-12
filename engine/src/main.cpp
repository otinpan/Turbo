#include <algorithm>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <memory>
#include <stdexcept>
#include <fstream>
#include <string>
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
  vk::raii::SurfaceKHR surface=nullptr; // connection between Vulkan and GLFW
  vk::raii::PhysicalDevice physicalDevice=nullptr; // physical device
  vk::raii::Device device=nullptr; // logical device
  vk::raii::Queue queue=nullptr;
  vk::raii::SwapchainKHR swapChain=nullptr;
  std::vector<vk::Image> swapChainImages;
  vk::SurfaceFormatKHR swapChainSurfaceFormat;
  vk::Extent2D swapChainExtent;
  std::vector<vk::raii::ImageView> swapChainImageViews;

  vk::raii::PipelineLayout pipelineLayout=nullptr;

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
    createSurface();
    pickPhysicalDevice();
    printPhysicalDevices();
    createLogicalDevice();
    createSwapChain();
    createImageViews();
    createGraphicsPipeline();
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


  void createInstance(){
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

  void createSurface(){
    VkSurfaceKHR _surface;
    if(glfwCreateWindowSurface(*instance,window,nullptr,&_surface)!=0){
      throw::std::runtime_error("failed to create window surfacee!");
    }

    surface=vk::raii::SurfaceKHR(instance,_surface);
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
      vk::PhysicalDeviceVulkan11Features,
      vk::PhysicalDeviceVulkan13Features,
      vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT>();

  bool supportsRequiredFeatures 
    = features.template get<vk::PhysicalDeviceVulkan11Features>().shaderDrawParameters &&
      features.template get<vk::PhysicalDeviceVulkan13Features>().dynamicRendering &&
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
    std::cout<<"search physical devices"<<std::endl;
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



  std::vector<const char*> getRequiredInstanceExtensions(){
    uint32_t glfwExtensionCount=0;
    auto glfwExtensions=glfwGetRequiredInstanceExtensions(&glfwExtensionCount);

    std::vector extensions(glfwExtensions,glfwExtensions+glfwExtensionCount);
    if(enableValidationLayers){
      extensions.push_back(vk::EXTDebugUtilsExtensionName);
    }

    return extensions;
  }

  void createLogicalDevice(){
    // find  queue family 
    std::vector<vk::QueueFamilyProperties> queueFamilyProperties=physicalDevice.getQueueFamilyProperties();
    // print queue family properties
    for (size_t i=0;i<queueFamilyProperties.size();++i){
      std::cout<<"Queue family "<<i<<": "<<vk::to_string(queueFamilyProperties[i].queueFlags)<<std::endl;
    }

    // get the first index into queueFamilyProperties that supports graphics and presentation
    uint32_t queueIndex=~0; // ~0 is the maximum value for uint32_t, which is used to indicate an invalid index
    for (uint32_t qfpIndex=0;qfpIndex<queueFamilyProperties.size();qfpIndex++){
      if((queueFamilyProperties[qfpIndex].queueFlags & vk::QueueFlagBits::eGraphics) &&
         physicalDevice.getSurfaceSupportKHR(qfpIndex,*surface)){
        queueIndex=qfpIndex;
        break;
      }
    }

    if(queueIndex==~0){
      throw std::runtime_error("could not find a queue for graphics and present -> terminating");
    }
    // device features to enable
    vk::StructureChain<
      vk::PhysicalDeviceFeatures2,
      vk::PhysicalDeviceVulkan11Features,
      vk::PhysicalDeviceVulkan13Features,
      vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT
    > 
    featureChain={
      {}, // vk::PhysicalDeviceFeatures2
      {.shaderDrawParameters=true}, // vk::PhysicalDeviceVulkan11Features
      {.dynamicRendering=true}, // vk::PhysicalDeviceVulkan13Features
      {.extendedDynamicState=true} // vk::PhysicalDeviceExtendedDynamicStateFeatureEXT
    };

    // create a Device
    float queuePriority=0.5f;
    vk::DeviceQueueCreateInfo deviceQueueCreateInfo{
      .queueFamilyIndex=queueIndex,
      .queueCount=1,
      .pQueuePriorities=&queuePriority
    };
    vk::DeviceCreateInfo deviceCreateInfo{
      .pNext=&featureChain.get<vk::PhysicalDeviceFeatures2>(),
      .queueCreateInfoCount=1,
      .pQueueCreateInfos=&deviceQueueCreateInfo,
      .enabledExtensionCount=static_cast<uint32_t>(requiredDeviceExtension.size()),
      .ppEnabledExtensionNames=requiredDeviceExtension.data()
    };

    device=vk::raii::Device(physicalDevice,deviceCreateInfo);
    queue=vk::raii::Queue(device, queueIndex,0);
  }

  void createSwapChain(){
    vk::SurfaceCapabilitiesKHR surfaceCapabilities=physicalDevice.getSurfaceCapabilitiesKHR(*surface);
    swapChainExtent=chooseSwapExtent(surfaceCapabilities);
    uint32_t minImageCount=chooseSwapMinImageCount(surfaceCapabilities);

    std::vector<vk::SurfaceFormatKHR> availableFormats=physicalDevice.getSurfaceFormatsKHR( surface );
    swapChainSurfaceFormat=chooseSwapSurfaceFormat(availableFormats);

    std::vector<vk::PresentModeKHR> availablePresentModes=physicalDevice.getSurfacePresentModesKHR(*surface);
    vk::PresentModeKHR presentMode=chooseSwapPresentMode(availablePresentModes);

    vk::SwapchainCreateInfoKHR swapChainCreateInfo{
      .surface=*surface,
      .minImageCount=minImageCount,
      .imageFormat=swapChainSurfaceFormat.format,
      .imageColorSpace=swapChainSurfaceFormat.colorSpace,
      .imageExtent=swapChainExtent,
      .imageArrayLayers=1,
      .imageUsage=vk::ImageUsageFlagBits::eColorAttachment,
      .imageSharingMode=vk::SharingMode::eExclusive,
      .preTransform=surfaceCapabilities.currentTransform,
      .compositeAlpha=vk::CompositeAlphaFlagBitsKHR::eOpaque,
      .presentMode=presentMode,
      .clipped=true
    };

    swapChain=vk::raii::SwapchainKHR(device,swapChainCreateInfo);
    swapChainImages=swapChain.getImages();
  }

  static uint32_t chooseSwapMinImageCount(vk::SurfaceCapabilitiesKHR const &surfaceCapabilities){
    // more than 3 images in the swap chain
    auto minImageCount=std::max(3u,surfaceCapabilities.minImageCount);
    if((0<surfaceCapabilities.maxImageCount) && (surfaceCapabilities.maxImageCount<minImageCount)){
      minImageCount=surfaceCapabilities.maxImageCount;
    }

    return minImageCount;
  }

  static vk::PresentModeKHR chooseSwapPresentMode(std::vector<vk::PresentModeKHR> const &availablePresentModes){
    assert(std::ranges::any_of(availablePresentModes,[](auto presentMode){return presentMode==vk::PresentModeKHR::eFifo;}));
    return std::ranges::any_of(
      availablePresentModes,
      [](auto presentMode){
        return presentMode==vk::PresentModeKHR::eMailbox;
      }
    ) ? vk::PresentModeKHR::eMailbox : vk::PresentModeKHR::eFifo;
  }

  static vk::SurfaceFormatKHR chooseSwapSurfaceFormat(std::vector<vk::SurfaceFormatKHR> const &availableFormats){
    assert(!availableFormats.empty());
    const auto formatIt=std::ranges::find_if(
      availableFormats,
      [](const auto &format){
        return format.format==vk::Format::eB8G8R8A8Srgb && format.colorSpace==vk::ColorSpaceKHR::eSrgbNonlinear;
      }
    );

    return formatIt!=availableFormats.end() ? *formatIt : availableFormats[0];
  }

  vk::Extent2D chooseSwapExtent(vk::SurfaceCapabilitiesKHR const &capabilities){
    // if pixel size doesn't match the window size, the swap chain will be stretched to fit the window, which can cause blurriness.
    if(capabilities.currentExtent.width!=std::numeric_limits<uint32_t>::max()){
      return capabilities.currentExtent;
    }
    int width,height;
    glfwGetFramebufferSize(window,&width,&height);
    return{
      std::clamp<uint32_t>(width,capabilities.minImageExtent.width,capabilities.maxImageExtent.width),
      std::clamp<uint32_t>(height,capabilities.minImageExtent.height,capabilities.maxImageExtent.height)
    };
  }

  void createImageViews(){
    assert(swapChainImageViews.empty());
    vk::ImageViewCreateInfo imageViewCreateInfo{
      .viewType=vk::ImageViewType::e2D,
      .format=swapChainSurfaceFormat.format,
      .subresourceRange={vk::ImageAspectFlagBits::eColor,0,1,0,1} 
    };

    for(auto &image:swapChainImages){
      imageViewCreateInfo.image=image;
      swapChainImageViews.emplace_back(device,imageViewCreateInfo);
    }
  }

  void createGraphicsPipeline(){
    vk::raii::ShaderModule shaderVertModule
      =createShaderModule(readFile(std::string(SHADER_BINARY_DIR) + "/vert.spv"));
    vk::raii::ShaderModule shaderFragModule
      =createShaderModule(readFile(std::string(SHADER_BINARY_DIR) + "/frag.spv"));

    vk::PipelineShaderStageCreateInfo vertShaderStageInfo{
      .stage=vk::ShaderStageFlagBits::eVertex,
      .module=shaderVertModule,
      .pName="vertMain"
    };

    vk::PipelineShaderStageCreateInfo fragShaderStageInfo{
      .stage=vk::ShaderStageFlagBits::eFragment,
      .module=shaderFragModule,
      .pName="fragMain"
    };

    vk::PipelineShaderStageCreateInfo shaderStages[]={
      vertShaderStageInfo,
      fragShaderStageInfo
    };

    vk::PipelineVertexInputStateCreateInfo vertexInputInfo;
    vk::PipelineInputAssemblyStateCreateInfo inputAssembly{
      .topology=vk::PrimitiveTopology::eTriangleList,
    };

    // need to specify to count each states
    vk::PipelineViewportStateCreateInfo viewportState{
      .viewportCount=1,
      .scissorCount=1
    };

    // specific parameters of the graphics pipeline are dynamically changed
    std::vector<vk::DynamicState> dynamicStates={vk::DynamicState::eViewport,vk::DynamicState::eScissor};
    vk::PipelineDynamicStateCreateInfo dynamicState{
      .dynamicStateCount=static_cast<uint32_t>(dynamicStates.size()),
      .pDynamicStates=dynamicStates.data()
    };

    vk::PipelineRasterizationStateCreateInfo rasterizer{
      .depthClampEnable=vk::False, // if true, fragments that are beyond the near and far planes are clamped to them as opposed to discarding them.
      .rasterizerDiscardEnable=vk::False, // if true, geometry never passes through the rasterizer state.
      .polygonMode=vk::PolygonMode::eFill, 
      // eFill: fill the area of the polygon with fragments.
      // eLine: polygon edges are drawn as lines.
      // ePoint: polygon vertices are drawn as lines.
      .cullMode=vk::CullModeFlagBits::eBack, // specify whether front- or back-facing (or both) polygons are culled (not drawn).
      .frontFace=vk::FrontFace::eClockwise, // specify the vertex order for the faces to be considered front-facing and can be clockwise or counterclockwise.
      .depthBiasEnable=vk::False, // if true, add biases to detpth values to resolve depth conflicts.
      .lineWidth=1.0f // the width of line.
    };

    // configures multisampling, which is one of the ways to perform anti-aliasing
    vk::PipelineMultisampleStateCreateInfo multisampling{
      .rasterizationSamples=vk::SampleCountFlagBits::e1, // if e1, no mulitsampling is used. if e4, use MSAA
      .sampleShadingEnable=vk::False, // if true, the final color of a pixel is determined by the shader invocation that covers the most samples.
    };

    // blending is the process of combining the color of a new fragment with the color of the existing pixel in the framebuffer.
    vk::PipelineColorBlendAttachmentState colorBlendAttachment{
      .blendEnable=vk::False, // if true, new color and old color are blended
      .colorWriteMask=vk::ColorComponentFlagBits::eR | vk::ColorComponentFlagBits::eG | 
      vk::ColorComponentFlagBits::eB | vk::ColorComponentFlagBits::eA // specify which color components are written to the framebuffer
    };

    vk::PipelineColorBlendStateCreateInfo colorBlending{
      .logicOpEnable=vk::False,
      .attachmentCount=1,
      .pAttachments=&colorBlendAttachment
    };

    vk::PipelineLayoutCreateInfo pipelineLayoutInfo{
      .setLayoutCount=0,
      .pushConstantRangeCount=0
    };
    pipelineLayout=vk::raii::PipelineLayout(device,pipelineLayoutInfo);

    vk::StructureChain<vk::GraphicsPipelineCreateInfo,vk::PipelineRenderingCreateInfo> pipelineCreateInfoChain={
      {
        .stageCount=2,
        .pStages=shaderStages,
        .pVertexInputState=&vertexInputInfo,
        .pInputAssemblyState=&inputAssembly,
        .pViewportState=&viewportState,
        .pRasterizationState=&rasterizer,
        .pMultisampleState=&multisampling,
        .pColorBlendState=&colorBlending,
        .pDynamicState=&dynamicState,
        .layout=pipelineLayout,
        .renderPass=nullptr
      },
      {
        .colorAttachmentCount=1,
        .pColorAttachmentFormats=&swapChainSurfaceFormat.format
      }
    };
  }

  [[nodiscard]] vk::raii::ShaderModule createShaderModule(const std::vector<char> &code) const{
    vk::ShaderModuleCreateInfo createInfo{
      .codeSize=code.size()*sizeof(char),
      .pCode=reinterpret_cast<const uint32_t*>(code.data())
    };
    vk::raii::ShaderModule shaderModule{device,createInfo};
    return shaderModule;
  }

  static VKAPI_ATTR vk::Bool32 VKAPI_CALL debugCallback(
      vk::DebugUtilsMessageSeverityFlagBitsEXT severity,
      vk::DebugUtilsMessageTypeFlagsEXT type,
      const vk::DebugUtilsMessengerCallbackDataEXT *pCallbackData,
      void*
  ){
    if(severity & (vk::DebugUtilsMessageSeverityFlagBitsEXT::eWarning |
                   vk::DebugUtilsMessageSeverityFlagBitsEXT::eError)){
      std::cerr<<"validation layer: type "<<vk::to_string(type)
               <<" mst: "<<pCallbackData->pMessage<<std::endl;
    }

    return VK_FALSE;
  }

  static std::vector<char> readFile(const std::string &filename){
    std::ifstream file(filename,std::ios::ate | std::ios::binary);
    if(!file.is_open()){
      throw std::runtime_error("failed to open file!");
    }
    std::vector<char> buffer(file.tellg());
    file.seekg(0,std::ios::beg);
    file.read(buffer.data(),static_cast<std::streamsize>(buffer.size()));
    return buffer;
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
