pub use collider::*;
mod collider {

    use std::fmt;

    use bevy::{gltf::Gltf, math::{Quat, Vec3,}, prelude::Component};
    use ncollide3d::{na::{Point3, Vector3}, query::{Ray, RayCast,}, shape::{Ball, Cuboid, Cylinder, TriMesh}};
    use obj::Obj;
    use easy_gltf::*;
    use crate::{Convert, RayCastResult, SnowFlake};

    #[derive(Clone)]
    pub enum ColliderType {
        Box{cuboid : Cuboid<f32>},
        Sphere{ball : Ball<f32>},
        Cylinder{cylinder : Cylinder<f32>},
        Mesh{
            vertex_buffer : Vec<[f32; 3]>,
            index_buffer : Vec<[usize; 3]>,
            trimesh : TriMesh<f32>,
        },
    }

    #[derive(Clone)]
    #[derive(Component)]
    pub struct Collider {
        id : SnowFlake,
        extent : f32,
        collider : ColliderType,
    }

    impl Collider {
        pub fn new_box(id : SnowFlake, half_width : f32, half_height : f32, half_length : f32) -> Self {
            let _box = Cuboid::<f32>::new(Vector3::new(half_width, half_height, half_length));
            Self {
                id,
                extent : mathfu::D3::distance((0.0, 0.0, 0.0), (half_width, half_height, half_length)),
                collider : ColliderType::Box{cuboid : _box},

            }
        }
        pub fn new_sphere(id : SnowFlake, radius : f32) -> Self {
            let sphere = Ball::<f32>::new(radius);
            Self {
                id,
                extent : radius,
                collider : ColliderType::Sphere{ball : sphere},
            }
        }

        pub fn new_cylinder(id : SnowFlake, half_height : f32, radius : f32) -> Self {
            let cylinder = Cylinder::<f32>::new(half_height, radius);
            Self {
                id,
                extent : mathfu::D2::hypotenuse(half_height, radius) ,
                collider : ColliderType::Cylinder{cylinder},
            }
        }

        pub fn new_mesh(id : SnowFlake, vertices : &Vec<[f32; 3]>, indices : &Vec<[usize; 3]>) -> Result<Self, ColliderError> {
            let mut extent : f32 = 0.0;
            for i in 0..vertices.len() {
                let v = mathfu::Dx::distance_between(vec![0.0; 3], vertices[i].to_vec());
                if v > extent {
                    extent = v;
                }
            }
            if Self::verify_mesh_data(&vertices, &indices) {
                let tri = Self::generate_trimesh(&vertices, &indices);
                Ok(Self {
                    id,
                    extent,
                    collider : ColliderType::Mesh{vertex_buffer : vertices.clone(), index_buffer : indices.clone(), trimesh : tri},
                })
            } else {
                Err(ColliderError::ColliderConstructionError)
            }
        }

        pub fn from_object(entity : SnowFlake, obj : &Obj) -> Result<Self, ColliderError> {
            match obj.data.objects.get(0) {
                Some(x) => {
                    match x.groups.get(0) {
                        Some(y) => {
                            let i_buf = &y.polys;
                            let mut con_i_buf = Vec::<[usize; 3]>::with_capacity(i_buf.capacity());
                            for i in 0..i_buf.len() {
                                for a in 2..i_buf[i].0.len() {
                                    match (i_buf[i].0.get(a - 2), i_buf[i].0.get(a - 1), i_buf[i].0.get(a)) {
                                        (Some(x), Some(y), Some(z)) => {
                                            con_i_buf.push([x.0, y.0, z.0]);
                                        },
                                        _ => {
                                            return Err(ColliderError::ColliderConstructionError);
                                        }
                                    }
                                }
                            }
                            Self::new_mesh(entity, &obj.data.position, &con_i_buf)
                        },
                        None => {
                            Err(ColliderError::ObjectError)
                        }
                    }
                },
                None => {
                    Err(ColliderError::ObjectError)
                }
            }
        }

        pub fn from_obj_file(id : SnowFlake, path : &str) -> Result<Self, ColliderError> {
            match Obj::load(path) {
                Ok(x) => {
                    return Self::from_object(id, &x);
                },
                Err(_) => {
                    return Err(ColliderError::LoadError);
                }
            }
        }

        pub fn from_gltf(id : SnowFlake, gltf : &Model) -> Result<Self, ColliderError> {
            match gltf.indices() {
                Some(x) => {
                    let v_buf = gltf.vertices();
                    let i_buf = x;

                    let mut con_v_buf = Vec::<[f32; 3]>::with_capacity(v_buf.capacity());
                    let mut con_i_buf = Vec::<[usize; 3]>::with_capacity(i_buf.capacity());
                    for i in v_buf {
                        let vertex : [f32; 3] = [i.position.x, i.position.y, i.position.z];
                        con_v_buf.push(vertex);
                    }
                    for i in 2..i_buf.len() {
                        if (i + 1) % 3 != 0 {
                            continue;
                        }
                        con_i_buf.push([i_buf[i - 2], i_buf[i - 1], i_buf[i]]);
                    }
                    Self::new_mesh(id, &con_v_buf, &con_i_buf)
                },
                None => {
                    Err(ColliderError::ColliderConstructionError)
                }
            }
        }

        pub fn from_gltf_file(id : SnowFlake, path : &str) -> Result<Self, ColliderError> {
            match easy_gltf::load(path) {
                Ok(x) => {
                    match x.get(0) {
                        Some(y) => {
                            match y.models.get(0) {
                                Some(z) => {
                                    Self::from_gltf(id, z)
                                },
                                None => {
                                    Err(ColliderError::ObjectError)
                                }
                            }
                        },
                        None => {
                            Err(ColliderError::ObjectError)
                        }
                    }
                },
                Err(_) => {
                    Err(ColliderError::LoadError)
                }
            }
        }

        fn verify_mesh_data(vertex_buffer : &Vec<[f32; 3]>, index_buffer : &Vec<[usize; 3]>) -> bool {
            for i in 0..index_buffer.len() {
                if let (Some(_), Some(_), Some(_),) =  (vertex_buffer.get(index_buffer[i][0]), vertex_buffer.get(index_buffer[i][1]), vertex_buffer.get(index_buffer[i][2])) {

                } else {
                    return false;
                }
            }
            return true;
        }

        fn generate_trimesh(vertex_buffer : &Vec<[f32; 3]>, index_buffer : &Vec<[usize; 3]>) -> TriMesh<f32> {

            let mut con_v_buf = Vec::<Point3<f32>>::with_capacity(vertex_buffer.capacity());
            let mut con_i_buf = Vec::<Point3<usize>>::with_capacity(index_buffer.capacity());

            for i in 0..vertex_buffer.len() {
                let p : Point3<f32> = Point3::new(vertex_buffer[i][0], vertex_buffer[i][1], vertex_buffer[i][2]);
                con_v_buf.push(p);
            }

            for i in 0..index_buffer.len() {
                let face : Point3<usize> = Point3::new(index_buffer[i][0], index_buffer[i][1], index_buffer[i][2]);
                con_i_buf.push(face);
            }

            return TriMesh::new(con_v_buf, con_i_buf, None);
        }

        pub fn id(&self) -> SnowFlake {
            self.id
        }

        pub fn extent(&self) -> f32 {
            self.extent
        }

        pub fn collider(&self) -> &ColliderType {
            &self.collider
        }

        pub fn toi_with_ray(&self, ray : &Ray<f32>, isometry : (Vec3, Quat)) -> Option<RayCastResult> {
            match &self.collider {
                ColliderType::Box { cuboid } => {
                    if let Some(x) =cuboid.toi_with_ray(&isometry.convert(), ray, f32::MAX, true) {
                        Some(RayCastResult{
                            id : self.id,
                            point : ray.point_at(x).convert(),
                            len : x,
                        })
                    } else {
                        None
                    }
                }
                ColliderType::Sphere { ball } => {
                    match ball.toi_with_ray(&isometry.convert(), ray, f32::MAX, true) {
                        Some(x) => {
                            Some(RayCastResult{
                                id : self.id,
                                point : ray.point_at(x).convert(),
                                len : x
                            })
                        },
                        None => { None }
                    }
                },
                ColliderType::Cylinder { cylinder } => {
                    match cylinder.toi_with_ray(&isometry.convert(), ray, f32::MAX, true) {
                        Some(x) => {
                            Some(RayCastResult{
                                id : self.id,
                                point : ray.point_at(x).convert(),
                                len : x
                            })
                        },
                        None => { None }
                    }
                },
                ColliderType::Mesh { vertex_buffer : _, index_buffer : _, trimesh } => {
                    match trimesh.toi_with_ray(&isometry.convert(), ray, f32::MAX, true) {
                        Some(x) => {
                            Some(RayCastResult{
                                id : self.id,
                                point : ray.point_at(x).convert(),
                                len : x
                            })
                        },
                        None => { None }
                    }
                }
            }
        }
    }

    impl fmt::Debug for ColliderType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ColliderType::Box { cuboid } => {
                    f.debug_struct("Cuboid")
                        .field("half_extents", &cuboid.half_extents)
                    .finish()
                }
                ColliderType::Sphere { ball } => {
                    f.debug_struct("Ball")
                        .field("radius", &ball.radius)
                    .finish()
                },
                ColliderType::Cylinder { cylinder } => {
                    f.debug_struct("Cylinder")
                        .field("half_height", &cylinder.half_height)
                        .field("radius", &cylinder.radius)
                    .finish()
                },
                ColliderType::Mesh { vertex_buffer, index_buffer, trimesh : _ } => {
                    f.debug_struct("TriMesh")
                        .field("vertex_buffer", &vertex_buffer)
                        .field("index_buffer", &index_buffer)
                    .finish()
                },
            }
            // f.debug_struct("ColliderType")
            //  .field("type", &self.entity)
            //  .finish()
        }
    }

    impl fmt::Display for ColliderType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ColliderType::Box { cuboid } => {
                    write!(f, "Box: half_extents: {}", cuboid.half_extents)
                }
                ColliderType::Sphere { ball } => {
                    write!(f, "Sphere: radius: {}", ball.radius)
                },
                ColliderType::Cylinder { cylinder } => {
                    write!(f, "Cylinder: height: {}, radius: {}", cylinder.half_height * 2.0, cylinder.radius)
                },
                ColliderType::Mesh { vertex_buffer, index_buffer, trimesh : _ } => {
                    write!(f, "Trimesh: verticies: {}, faces: {}", vertex_buffer.len(), index_buffer.len())
                },
            }
        }
    }

    impl fmt::Debug for Collider {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Collider")
                .field("entity", &self.id)
                //.field("isometry", &self.isometry)
                .field("type", &self.collider)
                .finish()
        }
    }

    impl fmt::Display for Collider {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Entity: {:?}, type: {}", self.id, /*self.isometry, */self.collider)
        }
    }

    #[derive(Debug)]
    pub enum ColliderError {
        LoadError,
        ObjectError,
        ColliderConstructionError,
    }

    impl fmt::Display for ColliderError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::LoadError => {
                    write!(f, "File does not exist.")
                },
                Self::ObjectError => {
                    write!(f, "No objects to make a mesh collider for.")
                },
                Self::ColliderConstructionError => {
                    write!(f, "Failed constructing mesh collider from object file. Make sure there are no dangling lines.")
                },
            }
        }
    }
}

#[test]
fn atest() {
    //for a in 1..=50 {
        for i in 2..3 {
            println!("What? : {}, {}, {}", i - 2, i - 1, i);
        }
    //}
}