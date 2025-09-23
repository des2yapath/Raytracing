use crate::material::Material;
use crate::ray::Ray;
use glam::DVec3;
use std::ops::Range;
use std::sync::Arc;

#[derive(Clone, Copy, Default)]
pub struct AABB {
    pub min: DVec3,
    pub max: DVec3,
}

impl AABB {
    pub fn new(min: DVec3, max: DVec3) -> Self {
        Self { min, max }
    }

    pub fn hit(&self, ray: &Ray, interval: Range<f64>) -> bool {
        let mut t_min = interval.start;
        let mut t_max = interval.end;

        for a in 0..3 {
            let inv_d = 1.0 / ray.direction[a];
            let mut t0 = (self.min[a] - ray.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin[a]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = t0.max(t_min);
            t_max = t1.min(t_max);

            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
        let min = box0.min.min(box1.min);
        let max = box0.max.max(box1.max);
        AABB::new(min, max)
    }
}

pub struct HitRecord {
    pub point: DVec3,
    pub normal: DVec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: DVec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<AABB>;
}

pub type HittableList = Vec<Arc<dyn Hittable>>;

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord> {
        let mut closest_so_far = interval.end;
        let mut hit_record = None;

        for object in self.iter() {
            if let Some(temp_rec) = object.hit(ray, interval.start..closest_so_far) {
                closest_so_far = temp_rec.t;
                hit_record = Some(temp_rec);
            }
        }

        hit_record
    }

    fn bounding_box(&self) -> Option<AABB> {
        if self.is_empty() {
            return None;
        }

        let mut output_box: Option<AABB> = None;

        for object in self {
            if let Some(temp_box) = object.bounding_box() {
                output_box = Some(match output_box {
                    Some(b) => AABB::surrounding_box(b, temp_box),
                    None => temp_box,
                });
            } else {
                return None;
            }
        }

        output_box
    }
}
