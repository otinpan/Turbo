# Vulkan

## Overview

This renderer sends different kinds of data to the GPU in different ways.

- Uniform matrices are updated from the CPU every frame.
- Vertex and index data are uploaded once through staging buffers.
- Texture pixels are uploaded once through a staging buffer, then sampled by the shader.
- Depth and multisample color images are not filled by the CPU. They are GPU render targets.
- Swapchain images are the final presentation images shown on the window.

## Send Texture Image

Texture image data is loaded on the CPU from `assets/textures/viking_room.png`.

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
- how to push_constants
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


The descriptor set stores the actual resources:

```text
set 0, binding 0 -> uniform_buffer
set 1, binding 0 -> material_buffer (texture, sampler, ...)
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

For normal rendering, the renderer uses one primary command buffer per swapchain image and secondary command buffers for object draw commands.

Current flow:

```text
render()
-> acquire swapchain image index
-> update_command_buffer(image_index)
-> update_uniform_buffer(image_index)
-> submit primary command buffer
-> present
```

The primary command buffer owns the render pass:

```text
primary command buffer
-> begin render pass
-> execute secondary command buffers
-> end render pass
```

The render pass is started with:

```rust
vk::SubpassContents::SECONDARY_COMMAND_BUFFERS
```

That tells Vulkan that draw commands inside the render pass will be recorded in secondary command buffers.

Each secondary command buffer records the draw work for one `RenderObject`:

```text
secondary command buffer
-> inherit render pass/framebuffer
-> bind graphics pipeline
-> bind mesh vertex buffer
-> bind mesh index buffer
-> bind descriptor set for this swapchain image
-> push model matrix
-> push opacity
-> draw indexed
```

The renderer decides how many objects to draw with:

```rust
let draw_count = renderer.models.min(renderer.data.render_objects.len());
```

Then it records one secondary command buffer for each visible object:

```rust
(0..draw_count)
    .map(|i| update_secondary_command_buffer(renderer, image_index, i))
```

Inside `update_secondary_command_buffer`, `model_index` selects a `RenderObject`.

```rust
let object = &renderer.data.render_objects[model_index];
let mesh = &renderer.data.meshes[object.mesh_index];
```

`RenderObject` decides which mesh to draw and where to place it:

```text
RenderObject
  mesh_index -> data.meshes[mesh_index]
  transform  -> object model matrix
```

`Mesh` owns the GPU buffers used for drawing:

```text
Mesh
  vertex_buffer
  index_buffer
  index_count
```

So the current renderer no longer draws from a single `data.vertex_buffer` / `data.index_buffer`. It draws by looking up the `Mesh` referenced by each `RenderObject`.

The primary command buffer is reset and recorded every frame for the acquired swapchain image. The secondary command buffers are stored per swapchain image and per object index, then re-recorded after the command pool is reset.

## Per-Frame Data

In the current renderer, the main CPU-to-GPU data updated every frame is the uniform matrix.

```text
updated every frame:
  uniform buffer matrix data
  primary command buffer
  secondary command buffers for visible RenderObjects

uploaded once:
  mesh vertex buffers
  mesh index buffers
  texture image

written by GPU during rendering:
  depth image
  MSAA color image
  swapchain image
```

## Shader
### vertex shader
* (set=0, binding=0) view/proj <-> global_descriptor_set
* (push_constant) model <-> pipeline_layout
* (layout=0,1,2) position/color/tex_coord <-> vertex_input_layout

### fragment shader
* (set=1, binding=0) texture <-> material_descriptor_set
* (push_constant) material_color/


