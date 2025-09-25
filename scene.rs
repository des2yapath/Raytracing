use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::objects::mesh::Mesh;
use crate::objects::sphere::Sphere;
use crate::texture::{CheckerTexture, ImageTexture, SolidColor, Texture};
use glam::DVec3;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;



#[derive(Deserialize)]
pub struct SceneConfig {
    pub aspect_ratio: Option<f64>,
    pub camera: CameraDef,
    pub objects: Vec<ObjectDef>,
}

#[derive(Deserialize)]
pub struct CameraDef {
    lookfrom: DVec3,
    lookat: DVec3,
    vup: DVec3,
    vfov: f64,
    aperture: f64,
    focus_dist: f64,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum ObjectDef {
    #[serde(rename = "sphere")]
    Sphere(SphereDef),
    #[serde(rename = "mesh")]
    Mesh(MeshDef),
}

#[derive(Deserialize)]
struct SphereDef {
    center: DVec3,
    radius: f64,
    material: MaterialDef,
}

#[derive(Deserialize)]
struct MeshDef {
    path: String,
    material: MaterialDef,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum MaterialDef {
    #[serde(rename = "lambertian")]
    Lambertian { texture: TextureDef },
    #[serde(rename = "metal")]
    Metal {
        texture: TextureDef,
        fuzz: f64,
    },
    #[serde(rename = "dielectric")]
    Dielectric { index_of_refraction: f64 },
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum TextureDef {
    #[serde(rename = "solid_color")]
    SolidColor { color: DVec3 },
    #[serde(rename = "checker")]
    Checker {
        scale: f64,
        even: Box<TextureDef>,
        odd: Box<TextureDef>,
    },
    #[serde(rename = "image")]
    Image { path: String },
}

// --- Scene Construction Logic ---

pub struct Scene;

impl Scene {
    pub fn from_file(
        path: &str,
    ) -> Result<(SceneConfig, Camera, Arc<dyn Hittable>), Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let scene_def: SceneConfig = serde_json::from_reader(reader)?;

        let aspect_ratio = scene_def.aspect_ratio.unwrap_or(16.0 / 9.0);

        let camera = Camera::new(
            scene_def.camera.lookfrom,
            scene_def.camera.lookat,
            scene_def.camera.vup,
            scene_def.camera.vfov,
            aspect_ratio,
            scene_def.camera.aperture,
            scene_def.camera.focus_dist,
        );

        let mut objects = HittableList::new();
        for obj_def in &scene_def.objects {
            objects.push(parse_object(obj_def));
        }

        let world = Arc::new(BvhNode::new(objects));

        Ok((scene_def, camera, world))
    }
}

fn parse_object(obj_def: &ObjectDef) -> Arc<dyn Hittable> {
    match obj_def {
        ObjectDef::Sphere(s) => Arc::new(Sphere::new(
            s.center,
            s.radius,
            parse_material(&s.material),
        )),
        ObjectDef::Mesh(m) => Arc::new(Mesh::new(&m.path, parse_material(&m.material))),
    }
}

fn parse_material(mat_def: &MaterialDef) -> Arc<dyn crate::material::Material> {
    match mat_def {
        MaterialDef::Lambertian { texture } => {
            Arc::new(Lambertian::new(parse_texture(texture)))
        }
        MaterialDef::Metal { texture, fuzz } => {
            Arc::new(Metal::new(parse_texture(texture), *fuzz))
        }
        MaterialDef::Dielectric {
            index_of_refraction,
        } => Arc::new(Dielectric::new(*index_of_refraction)),
    }
}

fn parse_texture(tex_def: &TextureDef) -> Arc<dyn Texture> {
    match tex_def {
        TextureDef::SolidColor { color } => Arc::new(SolidColor::new(*color)),
        TextureDef::Checker { scale, even, odd } => Arc::new(CheckerTexture::new(
            *scale,
            parse_texture(even),
            parse_texture(odd),
        )),
        TextureDef::Image { path } => Arc::new(ImageTexture::new(path)),
    }
}
