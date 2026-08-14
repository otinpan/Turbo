# DIRECTIONS
## ECS
### Create Entity
When you want to create entity, you use `world.spawn()`.
```rust
pub fn spawn(
    &mut self,
    transform: Transform,
    mesh_renderer: Option<MeshRenderer>,
    camera: Option<Camera>,
    rotate_speed: Vec3,
) -> EntityId {
    let entity = self.registry.create();

    self.registry.add_component(entity, transform);
    self.registry.add_component(entity, Visibility::default());
    ...
```
`spawn()` create Entity and attach specified component.

### Create System
System is classified
* InputSystem: detecting input, this create InputCommand queue.
* CommandSystem: this handle input event from InputCommand queue.
* UpdateSystem: Update all system, that mean update Entity
* RenderSystem: Rendering

when you register system to `Scheduler`, you have to create struct and use trait designated like `UpdateSystem`.
```rust
impl UpdateSystem for RotatorSystem {
    fn update(&mut self, context: &mut UpdateContext<'_>) -> Result<()> {
        for (_, transform, rotator) in context.registry.query2_mut::<Transform, Rotator>() {
            transform.rotate(rotator.speed * context.delta_time);
        }

        Ok(())
    }
}
```
then add to scheduler.
```rust
scheduler.add_update_system("rotator",Box::new(RotatorSystem));
```
### Bind Input Key
when you bind key and system, you use `scheduler::bind_key()`.
firstly, you have to create system struct obtaining `Command` trait.
```rust
impl Command for DespawnLastCommand {
    fn id(&self) -> String {
        "despawn_last".to_string()
    }

    fn execute(&self, context: &mut CommandContext<'_>) -> Result<()> {
        let id = context.world.registry.entities().last().copied();

        if let Some(id) = id {
            context.world.despawn(id);
        }

        Ok(())
    }
}
```
then, you register this system to scheduler.
```rust
scheduler.bind_key(
    KeyCode::ArrowLeft,
    InputTrigger::Pressed,
    DespawnLastCommand,
);
```

## RENDERING
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

### Skybox
When you use skybox, you have to create `MeshHandle`, `SkyboxTextureHandle` and `SkyboxRenderer`.

1. create `MeshHandle` using `create_skybox_mesh`
2. load square texture (height=width) and create `SkyboxTextureHandle`
3. sync `SkyboxRenderer` in renderer with `set_skybox`

```rust
unsafe fn create_skybox_mesh(renderer: &mut VulkanRenderer) -> Result<MeshHandle>
```

```rust
unsafe fn load_skybox_textures(renderer: &mut VulkanRenderer) -> Result<HashMap<String,SkyboxTextureHandle>>
```

```rust
pub unsafe fn set_skybox(&mut self, texture: SkyboxTextureHandle) -> Result<()> {
    let mesh = self
        .skybox_mesh
        .ok_or_else(|| anyhow!("Skybox mesh has not been created."))?;

    self.renderer.set_skybox(mesh, texture)
}
```

renderer
```rust
pub unsafe fn set_skybox(
    &mut self,
    mesh: MeshHandle,
    texture: SkyboxTextureHandle,
) -> Result<()> {
    if mesh.vertex_layout != VertexLayout::Skybox {
        return Err(anyhow!("Skybox mesh must use VertexLayout::Skybox."));
    }

    if self.data.skybox_textures.get(texture.0).is_none() {
        return Err(anyhow!("Skybox texture index out of range: {}", texture.0));
    }

    self.data.skybox = Some(RenderSkybox {
        mesh,
        texture,
        is_visible: true,
    });
    ...
```