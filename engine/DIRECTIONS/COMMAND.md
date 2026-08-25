# Command

A command is a user-defined action that runs with `CommandContext`.

```rust
pub trait Command {
    fn id(&self) -> String;
    fn execute(&self, context: &mut CommandContext<'_>) -> Result<()>;
}
```

Bind a command to an input trigger through `Scheduler`.

```rust
scheduler.bind_key(
    KeyCode::ArrowLeft,
    InputTrigger::Pressed,
    DespawnLastCommand,
);
```

`CommandContext` is the public API surface for commands. It owns no game data; it only gives temporary access to the operations allowed during the command stage.

### Entity

```rust
pub fn spawn(&mut self) -> EntityId
```

Creates a new empty entity.

```rust
pub fn despawn(&mut self, entity: EntityId) -> bool
```

Despawns an entity. If it has a `MeshRenderer`, the mesh reference count is released and mesh destruction is queued when needed.

```rust
pub fn despawn_last(&mut self) -> bool
```

Despawns the last registered entity.

```rust
pub fn entities(&self) -> &[EntityId]
```

Returns all currently registered entities.

```rust
pub fn is_entity_registered(&self, entity: EntityId) -> bool
```

Returns whether the entity exists.

```rust
pub fn entity_count(&self) -> usize
```

Returns the number of registered entities.

### Component

```rust
pub fn add_component<T: Component>(&mut self, entity: EntityId, component: T) -> bool
```

Adds or replaces a component on an entity.

```rust
pub fn remove_component<T: Component>(&mut self, entity: EntityId, component: T) -> Option<T>
```

Removes a component from an entity.

```rust
pub fn get_component<T: Component>(&mut self, entity: EntityId) -> Option<&T>
```

Returns an immutable component reference.

```rust
pub fn get_component_mut<T: Component>(&mut self, entity: EntityId) -> Option<&mut T>
```

Returns a mutable component reference.

```rust
pub fn has_component<T: Component>(&self, entity: EntityId) -> bool
```

Returns whether an entity has a component.

```rust
pub fn get_component_pool<T: Component>(&self) -> Option<&ComponentPool<T>>
pub fn get_component_pool_mut<T: Component>(&mut self) -> Option<&mut ComponentPool<T>>
```

Returns the full component pool for advanced iteration.

### Query

```rust
pub fn query2<A, B>(&self) -> Box<dyn Iterator<Item = (EntityId, &A, &B)> + '_>
```

Iterates entities that have both components as immutable references.

```rust
pub fn query2_mut<A, B>(&mut self) -> Box<dyn Iterator<Item = (EntityId, &mut A, &B)> + '_>
```

Iterates entities with mutable `A` and immutable `B`.

```rust
pub fn query2_mut_mut<A, B>(&mut self) -> Box<dyn Iterator<Item = (EntityId, &mut A, &mut B)> + '_>
```

Iterates entities with mutable `A` and mutable `B`.

### Name And Tags

```rust
pub fn find_entity_by_name(&self, name: &str) -> Option<EntityId>
```

Finds an entity by unique name.

```rust
pub fn set_name(&mut self, entity: EntityId, name: &str) -> bool
pub fn remove_name(&mut self, entity: EntityId) -> bool
```

Adds or removes an entity name.

```rust
pub fn set_tags<const N: usize>(&mut self, entity: EntityId, tags: [&str; N]) -> bool
pub fn remove_tags(&mut self, entity: EntityId) -> bool
pub fn remove_tag(&mut self, entity: EntityId, tag: &str) -> bool
```

Adds, removes, or edits tags on an entity.

```rust
pub fn get_entities_by_tag(&self, tag: &str) -> Vec<EntityId>
```

Returns all entities that have the tag.

```rust
pub fn get_all_named_entities(&self) -> Vec<(String, EntityId)>
pub fn get_all_taged_entities(&self) -> Vec<(String, EntityId)>
```

Returns debug-friendly lists of registered names and tags.

### Input

```rust
pub fn mouse_position(&self) -> Vec2
pub fn window_size(&self) -> Vec2
```

Returns current input state needed by commands.

```rust
pub fn positions(&self) -> &[Vec3]
```

Returns predefined spawn positions used by the sample commands.

### Model

```rust
pub fn spawn_model(
    &mut self,
    model_name: &str,
    transform: Transform,
    material: Material,
) -> Result<EntityId>
```

Spawns an entity from an already loaded model. It retains the mesh asset and adds `Transform`, `MeshRenderer`, `Visibility`, and model tags.

```rust
pub fn model_asset_id(&self, model_name: &str) -> Result<MeshAssetId>
```

Returns the mesh asset id for a loaded model.

### Primitive

```rust
pub fn spawn_primitive_from_mesh(
    &mut self,
    asset_id: MeshAssetId,
    material: Material,
    transform: Transform,
) -> Result<EntityId>
```

Spawns a primitive entity from an existing primitive mesh asset.

```rust
pub fn primitive_asset_id(
    &self,
    primitive_type: PrimitiveType,
    vertex_layout: VertexLayout,
) -> Option<MeshAssetId>
```

Finds a primitive mesh asset by primitive type and vertex layout.

```rust
pub fn update_primitive_mesh(
    &mut self,
    primitive_type: PrimitiveType,
    vertex_layout: VertexLayout,
    shape: PrimitiveShape,
)
```

Queues a render command to update an existing primitive mesh on the render stage.

```rust
pub fn primitive_type_from_asset_id(&self, asset_id: MeshAssetId) -> Option<PrimitiveType>
pub fn vertex_layout_from_asset_id(&self, asset_id: MeshAssetId) -> Option<VertexLayout>
```

Returns primitive metadata for a primitive mesh asset.

```rust
pub fn enqueue_spawn_shape(
    &mut self,
    shape: PrimitiveShape,
    transform: Transform,
    material: Material,
    auto_release: bool
) -> Result<EntityId>
```

This method create new mesh and spawn new entity. When you want to create primitive from `PrimitiveShape` and `Material`, you use it.

<details>
<summary>example</summary>

```rust
pub struct CreatePrimitiveCommand {
    pub primitive_shape: PrimitiveShape,
    pub transform: Transform,
    pub color: Vector3<f32>,
    pub alpha: f32,
    pub texture: Option<&'static str>,
    pub pipeline_key: PipelineKey,
    pub auto_release: bool,
}

impl Command for CreatePrimitiveCommand {
    fn id(&self) -> String {
        format!(
            "create_primitive:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}",
            self.primitive_shape,
            self.transform,
            self.color,
            self.alpha,
            self.texture,
            self.pipeline_key
        )
    }

    fn execute(&self, context: &mut CommandContext<'_>) -> Result<()> {
        let texture = match self.texture {
            Some(name) => context.texture(name)?,
            None => context.default_texture(),
        };

        let material = Material {
            color: self.color,
            alpha: self.alpha,
            use_texture: self.texture.is_some(),
            texture,
            pipeline_key: self.pipeline_key,
        };

        context.enqueue_spawn_shape(
            self.primitive_shape.clone(),
            self.transform.clone(),
            material,
            self.auto_release,
        )?;

        Ok(())
    }
}
```
```rust
scheduler.bind_key(
    KeyCode::Digit2,
    InputTrigger::Pressed,
    CreatePrimitiveCommand {
        primitive_shape: PrimitiveShape::Triangle {
            points: [
                vec3(0.0, 2.0, -0.3),
                vec3(-7.0, 2.0, 0.3),
                vec3(-2.0, 2.0, 1.0),
            ],
            color: vec3(1.0, 1.0, 1.0),
        },
        transform: Transform {
            position: vec3(-5.0, 2.0, 0.0),
            ..Default::default()
        },
        color: vec3(1.0, 1.0, 0.0),
        alpha: 1.0,
        texture: Some("face"),
        pipeline_key: PipelineKey::DebugLine3D,
        auto_release: true,
    },
);
```
</details>

```rust
fn spawn_triangle_3d(
    &mut self,
    p0: Vec3,
    p1: Vec3,
    p2: Vec3,
    color: Vec3,
    alpha: f32,
    texture: Option<&str>,
    pipeline_key: PipelineKey,
) -> Result<EntityId> 
```
```rust
fn spawn_triangle_2d(
    &mut self,
    p0: Vec2,
    p1: Vec2,
    p2: Vec2,
    color: Vec3,
    alpha: f32,
    texture: Option<&str>,
) -> Result<EntityId> 
```
```rust
fn spawn_rectangle_3d(
    &mut self,
    pos: Vec3,
    width: f32,
    height: f32,
    rotation: Vec3,
    color: Vec3,
    alpha: f32,
    texture: Option<&str>,
    pipeline_key: PipelineKey,
) -> Result<EntityId> 
```
```rust
fn spawn_rectangle_2d(
    &mut self,
    pos: Vec2,
    width: f32,
    height: f32,
    rotation: f32,
    color: Vec3,
    alpha: f32,
    texture: Option<&str>,
) -> Result<EntityId> 
```
```rust
fn spawn_cube_3d(
    &mut self,
    pos: Vec3,
    length: f32,
    rotation: Vec3,
    color: Vec3,
    alpha: f32,
    texture: Option<&str>,
    pipeline_key: PipelineKey,
) -> Result<EntityId> 
```

```rust
fn spawn_circle_3d(
    &mut self,
    pos: Vec3,
    radius: f32,
    segments: u32,
    color: Vec3,
    alpha: f32,
    texture: Option<&str>,
    pipeline_key: PipelineKey,
) -> Result<EntityId>
```

```rust
fn spawn_circle_2d(
    &mut self,
    pos: Vec2,
    radius: f32,
    segments: u32,
    color: Vec3,
    alpha: f32,
    texture: Option<&str>,
) -> Result<EntityId> 
```

```rust
fn spawn_polygon_3d(
    &mut self,
    points: Vec<Vec3>,
    color: Vec3,
    alpha: f32,
    texture: Option<&str>,
    pipeline_key: PipelineKey,
) -> Result<EntityId> 
```

```rust
fn spawn_polygon_2d(
    &mut self,
    points: Vec<Vec2>,
    color: Vec3,
    alpha: f32,
    texture: Option<&str>,
) -> Result<EntityId> 
```

```rust
fn spawn_sphere_3d(
    &mut self,
    center: Vec3,
    radius: f32,
    rings: u32,
    segments: u32,
    color: Vec3,
    alpha: f32,
    texture: Option<&str>,
    pipeline_key: PipelineKey,
) -> Result<EntityId> 
```

```rust
fn spawn_line_3d(
    &mut self,
    pos0: Vec3,
    pos1: Vec3,
    color: Vec3,
    alpha: f32,
) -> Result<EntityId>
```

```rust
fn spawn_line_2d(
    &mut self,
    pos0: Vec2,
    pos1: Vec2,
    color: Vec3,
    width: f32,
    alpha: f32,
) -> Result<EntityId> 
```

These method create new mesh and spawn primitive easily. They call `enqueue_spawn_primitive()` in themselves.

<details>
<summary>example</summary>

```rust
    struct CreateTriangle{
        p0: Vec3,
        p1: Vec3,
        p2: Vec3,
        color: Vec3,
        alpha: f32,
        texture: Option<&'static str>,
        pipeline_key: PipelineKey,
    }
    impl Command for CreateTriangle{
        fn id(&self) -> String{
            format!(
                "create_triangle"
            )
        }
        fn execute(&self, context: &mut CommandContext<'_>) -> Result<()>{
            context.spawn_triangle_3d(
                self.p0,self.p1,self.p2,
                self.color, self.alpha,
                self.texture,
                self.pipeline_key,
            )?;

            Ok(())
        }
    }
    scheduler.bind_key(
        KeyCode::Digit1,
        InputTrigger::Pressed,
        CreateTriangle{
            p0: vec3(0.0,2.0,-0.3),
            p1: vec3(-7.0,2.0,0.3),
            p2: vec3(-2.0,2.0,1.0),
            color: vec3(1.0,1.0,0.0),
            alpha: 1.0,
            texture: Some("face"),
            pipeline_key: PipelineKey::Lit3D,
        },
    );
```



### Texture

```rust
pub fn texture(&self, texture_name: &str) -> Result<TextureHandle>
```

Returns a loaded texture handle by name.

```rust
pub fn default_texture(&self) -> TextureHandle
pub fn default_skybox_texture(&self) -> SkyboxTextureHandle
```

Returns default texture handles.

### Debug

```rust
pub fn mesh_assets(&self) -> impl Iterator<Item = (MeshAssetId, &MeshAsset)>
```

Returns existing mesh assets for debugging.
