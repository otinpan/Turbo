# DIRECTIONS
## Flow
handle
```rust
pub struct MeshHandle{
  index: usize,
  vertex_pipeline: VertexPipeline,
};
```
```rust
pub struct TextureHandle(pub usize);
```
```rust
pub enum PipelineKey {
    Mesh3D,
    DebugLine3D,
}
```

engine
```rust
pub struct WorldObject {
    pub id: EntityId,
    pub transform: Transform,
    pub mesh_renderer: Option<MeshRenderer>,
    pub camera: Option<CameraComponent>,
    rotate_speed: Vec3,
    is_visible: bool,
}
```
```rust
pub struct MeshRenderer {
    pub mesh: MeshHandle,
    pub material: Material,
}
```

```rust
pub struct Material {
    pub color: Vec3,
    pub use_texture: bool,
    pub texture: TextureHandle,
    pub pipeline_key: PipelineKey,
}
```

renderer_vulkan
```rust
pub struct RenderItem {
    pub mesh_index: MeshHandle,
    pub transform: Transform,
    // material
    pub material_color: cgmath::Vector3<f32>,
    pub use_texture: bool,
    pub texture_index: TextureHandle, // use Texture from VulkanData::textures
    pub pipeline_key: PipelineKey,
    pub is_visible: bool,
}
```

```rust
pub struct GraphicsPipeline {
    pub key: PipelineKey,
    pub layout: vk::PipelineLayout,
    pub pipeline: vk::Pipeline,
}
```

```rust
pub struct Texture {
    pub image: vk::Image,
    pub image_memory: vk::DeviceMemory,
    pub image_view: vk::ImageView,
    pub mip_levels: u32,
}
```

SourceMesh has all vertex data.
```rust
pub struct SourceMesh{
    pub vertices: Vec<SourceVertex>,
    pub indices: Vec<u32>,
    pub topology: SourceTopology,
}
```

MeshData is created from SourceMesh
```rust
pub struct MeshData<V> {
    pub vertices: Vec<V>,
    pub indices: Vec<u32>,
}
```

```rust
pub struct Mesh {
    pub vertex_buffer: vk::Buffer,
    pub vertex_buffer_memory: vk::DeviceMemory,
    pub vertex_buffer_size: vk::DeviceSize,

    pub index_buffer: vk::Buffer,
    pub index_buffer_memory: vk::DeviceMemory,
    pub index_buffer_size: vk::DeviceSize,

    pub index_count: u32,
    pub vertex_layout: VertexLayout,
}
```

### Create Mesh
Models and primitives are loaded by `load_mesh_from_model` and `load_mesh_from_primitive`.
```rust
load_mesh_from_model(<path>)
  load_model_source(<path>) -> SourceMesh
    convert -> MeshData<T>
      load_model_from_data(SourceMesh,VertexLayout) -> MeshHandle 
```

```rust
load_mesh_from_vertices<V>(vertices<V>, indices,VertexLayout)
  load_mesh_from_data(MeshData{vertices,indices},layout) -> MeshHandle
```

### Create Pipeline
Pipeline connects shader and vulkan. It discribes shaders, `push_constant` ranges and offsets. In addition,
- input assembly
- viewport and scissor
- rasterizer
- multisampling state
- depth/stencil state
- color blending
- pipeline layout and descriptor set layout

```rust
create_mesh3d_pipeline()
```
then created pipeline is pushed in `VulkanData::pipelines: Vec<GraphicsPipeline>`. `update_secomdary_command_buffer` in `command.rs` use it.

### Create Descriptor Set
Descriptor set connects uniform buffer and texture to shader. It describes `binding`, which is resource of uniform buffer, texture sample, and more. They are typed 
* global_descriptor_set
* material_descriptor_set

global_descriptor_set is used for uniform buffer, which is updated every frame. material_descriptor_set is used for texture, which is updated when material is changed.

### Command Buffer
pipeline and descriptor set is linked in `update_secondary_command_buffer` in `command.rs`. This function connects them with
```rust
    let sets = [global_set, material_set];
    renderer.device.cmd_bind_descriptor_sets(
        command_buffer,
        vk::PipelineBindPoint::GRAPHICS,
        pipeline.layout,
        0,
        &sets,
        &[],
    );
  ```

  and push shader to model matrix using
```rust
    renderer.device.cmd_push_constants(
        command_buffer,
        pipeline.layout,
        vk::ShaderStageFlags::VERTEX,
        0,
        model_bytes,
    );
```

### Spawn Object
interfaces are 
```rust
pub fn spawn_primitive_from_mesh(
    world: &mut World,
    mesh: MeshHandle,
    material: Material,
    transform: Transform,
) -> Result<Entity>{
  world.spawn(
    ...,
    Some(MeshRenderer::new(mesh, material)?),
  )
}
```
and
```rust
    pub fn spawn(
        &mut self,
        transform: Transform,
        mesh_renderer: Option<MeshRenderer>,
        camera: Option<CameraComponent>,
        rotate_speed: Vec3,
    ) -> EntityId {

    }
```

The consistency between meshdata and material, whitch means linking vertex and pipeline (binding to vertex layout), are guaranteed by `new` interface.

The reason that `Material` has `PipelineKey` is single vertex layout will match multiple pipeline.

```
VertexLayout: Mesh3D
-> PipelineLayout: Mesh3D, Lit3D, Skybox ...
```