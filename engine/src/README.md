# Vulkan

## Overview

This renderer sends different kinds of data to the GPU in different ways.

- Uniform matrices are updated from the CPU every frame.
- Vertex and index data are uploaded once through staging buffers.
- Texture pixels are uploaded once through a staging buffer, then sampled by the shader.
- Depth and multisample color images are not filled by the CPU. They are GPU render targets.
- Swapchain images are the final presentation images shown on the window.

## Send Texture Image

Texture image data is loaded on the CPU from `src/assets/viking_room.png`.

The final texture image is created with `DEVICE_LOCAL` memory, so it is fast for the GPU to read but is not written directly by the CPU. Because of that, the upload uses a staging buffer.

Flow:

```text
CPU loads PNG pixels
-> CPU writes pixels into a HOST_VISIBLE staging buffer
-> command buffer records cmd_copy_buffer_to_image
-> GPU copies staging buffer into DEVICE_LOCAL texture image
-> mipmaps are generated
-> texture_image_view + texture_sampler are written to descriptor set binding 1
-> fragment shader samples the texture
```

Important objects:

- `texture_image`: GPU image containing the texture pixels.
- `texture_image_memory`: memory bound to `texture_image`.
- `texture_image_view`: tells Vulkan how the shader sees the image.
- `texture_sampler`: tells Vulkan how to read the image, such as filtering, repeat mode, and mipmap behavior.

The sampler is not the image itself. It is the reading rule for the image.

## Depth Image

The depth image is not uploaded from the CPU.

It is created as an empty GPU image with `DEPTH_STENCIL_ATTACHMENT` usage. During rendering, the GPU writes depth values into it.

Flow:

```text
create depth_image on GPU
-> create depth_image_view
-> attach it to the framebuffer
-> render pass clears it to depth = 1.0
-> GPU writes depth values during depth testing
```

Depth is normally worth using in 3D rendering because it gives correct front/back ordering and can reject hidden fragments.

## Multisampling Image

The multisample color image is also not uploaded from the CPU.

It is a GPU render target used before presenting to the screen. Fragment shader output is written into this MSAA image first. Then Vulkan resolves it into the single-sample swapchain image.

Flow:

```text
fragment shader output
-> MSAA color_image
-> resolve attachment
-> swapchain image
-> present
```

MSAA quality is controlled by `data.msaa_samples`. The sample count must match between:

- render pass color/depth attachments
- graphics pipeline multisample state
- MSAA color image
- depth image
- framebuffer attachments

Changing MSAA at runtime usually requires recreating the dependent render resources, such as render pass, pipeline, color image, depth image, framebuffers, and command buffers.

## Swapchain

The swapchain is the set of images used for presenting rendered results to the window.

The renderer does not draw directly to the monitor. Instead, it gets one swapchain image, renders into it or resolves into it, and then presents it.

Flow:

```text
acquire swapchain image index
-> use framebuffer[index]
-> render
-> present swapchain image
```

Swapchain images are usually single-sample images. With MSAA, rendering first goes into a separate multisample color image, then resolves into the swapchain image.

## RenderPass and Pipeline

Render pass and pipeline are different Vulkan objects.

Short version:

```text
RenderPass = where rendering goes, and how attachments are handled
Pipeline   = how vertices/fragments are processed while rendering
```

### RenderPass

The render pass describes the attachment layout for a draw pass.

It answers questions like:

- attachment formats
- color/depth/resolve attachment count
- sample count for each attachment
- whether an attachment is cleared or loaded
- whether an attachment is stored after rendering
- initial and final image layouts
- whether MSAA color is resolved into a swapchain image

In this renderer, the render pass has three attachments:

```text
attachment 0: MSAA color attachment
attachment 1: depth attachment
attachment 2: resolve attachment, the swapchain image
```

So the render pass says:

```text
draw into an MSAA color image
use a depth image for depth testing
resolve the MSAA result into the swapchain image
```

The render pass does not choose the concrete images by itself. It only defines the expected attachment structure.

### Pipeline

The graphics pipeline describes how drawing is executed.

It contains settings such as:

- vertex shader
- fragment shader
- vertex input layout
- input assembly
- viewport and scissor
- rasterizer
- multisampling state
- depth/stencil state
- color blending
- pipeline layout and descriptor set layout

The pipeline is created for a specific render pass and subpass:

```rust
.render_pass(data.render_pass)
.subpass(0)
```

That means if the render pass changes, the pipeline usually needs to be recreated too.

### When To Recreate

Change only the pipeline when the render target structure stays the same but the drawing style changes.

Examples:

```text
change shader
change culling
change blend mode
change depth test behavior
change vertex format
```

Change the render pass, and usually the pipeline too, when the attachment structure changes.

Examples:

```text
enable or disable depth attachment
enable or disable MSAA
change MSAA sample count
change color/depth format
change resolve attachment
```

For MSAA changes, the dependent resources are usually recreated together:

```text
render_pass
pipeline
color_image / color_image_view
depth_image / depth_image_view
framebuffers
command_buffers
```

## Framebuffer

The framebuffer connects the render pass attachment slots to actual image views.

For each swapchain image, the renderer creates one framebuffer:

```text
framebuffer[i]
  attachment 0 -> color_image_view
  attachment 1 -> depth_image_view
  attachment 2 -> swapchain_image_views[i]
```

During rendering, the acquired swapchain image index selects the framebuffer.

## Send Uniform buffer

Uniform buffers store small data that changes often. In this renderer, the uniform buffer contains:

```text
model matrix
view matrix
projection matrix
```

Unlike texture, vertex, and index data, the uniform matrix is updated every frame.

The uniform buffer memory is created with:

```text
HOST_VISIBLE | HOST_COHERENT
```

So the CPU can write to it directly with `map_memory`.

Flow:

```text
CPU calculates model/view/proj
-> CPU maps uniform buffer memory
-> CPU copies UniformBufferObject into it
-> command buffer binds descriptor_set
-> vertex shader reads descriptor set binding 0
```

The command buffer does not contain the matrix data itself. It only binds the descriptor set. The descriptor set points to the current uniform buffer.

## Descriptor Set

A descriptor set is the resource table used by shaders.

The descriptor set layout says:

```text
binding 0 -> uniform buffer
binding 1 -> combined image sampler
```

The descriptor set stores the actual resources:

```text
binding 0 -> uniform_buffers[i]
binding 1 -> texture_image_view + texture_sampler
```

The command buffer binds the descriptor set before drawing. Then the shaders can read the uniform buffer and sample the texture.

## Command Buffer

Command buffers contain GPU commands.

Examples:

- begin render pass
- bind pipeline
- bind vertex buffer
- bind index buffer
- bind descriptor set
- draw indexed
- copy staging buffer to texture image

For texture upload, a temporary one-time command buffer is used for `cmd_copy_buffer_to_image`.

For normal rendering, command buffers are recorded ahead of time and submitted every frame with `queue_submit`.

## Per-Frame Data

In the current renderer, the main CPU-to-GPU data updated every frame is the uniform matrix.

```text
updated every frame:
  uniform buffer matrix data

uploaded once:
  vertex buffer
  index buffer
  texture image

written by GPU during rendering:
  depth image
  MSAA color image
  swapchain image
```
