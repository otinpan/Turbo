use std::collections::{HashMap};
use cgmath::{Vector3};
use crate::{PrimitiveMesh};
use renderer_vulkan::{MeshHandle,TextureHandle,SkyboxTextureHandle};

pub type Vec3=Vector3<f32>;

pub struct Resources {
    pub models: HashMap<String, MeshHandle>,
    pub textures: HashMap<String, TextureHandle>,
    pub primitive_meshes: Vec<PrimitiveMesh>,
    pub skybox_mesh: Option<MeshHandle>,
    pub skybox_textures: HashMap<String, SkyboxTextureHandle>,
}

impl Resources{
    pub fn new(
        models: HashMap<String,MeshHandle>,
        textures: HashMap<String,TextureHandle>,
        primitive_meshes: Vec<PrimitiveMesh>,
        skybox_mesh: Option<MeshHandle>,
        skybox_textures: HashMap<String,SkyboxTextureHandle>,
    ) -> Self{
        Self { models, textures, primitive_meshes, skybox_mesh, skybox_textures }
    }
}