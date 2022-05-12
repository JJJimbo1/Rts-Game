pub use conversion::*;
mod conversion {

    // use amethyst::core::{
    //     geometry::Ray as AmRay,
    //     math::{
    //         Point3 as AmPoint3, Vector3 as AmVector3, Isometry3 as AmIsometry3,
    //     },
    // };

    use bevy::math::{
        Vec3, Quat,
    };

    use ncollide3d::{
        query::Ray as NcRay,
        na::{
            Point3, Vector3, Isometry3,
        },
    };

    pub trait Convert<T> {
        fn convert(&self) -> T;
    }

    impl Convert<Vec3> for Point3<f32> {
        fn convert(&self) -> Vec3 {
            Vec3::new(self.x, self.y, self.z)
        }
    }

    impl Convert<Point3<f32>> for Vec3 {
        fn convert(&self) -> Point3<f32> {
            Point3::<f32>::new(self.x, self.y, self.z)
        }
    }
    
    impl Convert<Vec3> for Vector3<f32> {
        fn convert(&self) -> Vec3 {
            Vec3::new(self.x, self.y, self.z)
        }
    }

    impl Convert<Vector3<f32>> for Vec3 {
        fn convert(&self) -> Vector3<f32> {
            Vector3::<f32>::new(self.x, self.y, self.z)
        }
    }


    impl Convert<(Vec3, Quat)> for Isometry3<f32> {
        fn convert(&self) -> (Vec3, Quat) {
            let rotation : Quat = match self.rotation.axis_angle() {
                Some(x) => {
                    Quat::from_axis_angle(Vec3::new(x.0.x, x.0.y, x.0.z), x.1)
                },
                None => {
                    Quat::IDENTITY
                }
            };
            (self.translation.vector.convert(), rotation)
        }
    }

    // impl Convert<AmRay<f32>> for NcRay<f32> {
    //     fn convert(&self) -> AmRay<f32> {
    //         AmRay {
    //             origin : self.origin.convert(),
    //             direction : self.dir.convert(),
    //         }
    //     }
    // }

    impl Convert<Isometry3<f32>> for (Vec3, Quat) {
        fn convert(&self) -> Isometry3<f32> {
            let rotation : Vector3<f32> = {
                let r = self.1.to_axis_angle();
                let rot = Vector3::new(r.0.x, r.0.y, r.0.z) * r.1;
                rot
            };
            Isometry3::<f32>::new(self.0.convert(), rotation)
        }
    }

    // impl Convert<NcRay<f32>> for AmRay<f32> {
    //     fn convert(&self) -> NcRay<f32> {
    //         NcRay {
    //             origin : self.origin.convert(),
    //             dir : self.direction.convert(),
    //         }
    //     }
    // }
}