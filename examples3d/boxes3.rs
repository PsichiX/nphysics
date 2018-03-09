extern crate nalgebra as na;
extern crate ncollide;
extern crate nphysics3d;
extern crate nphysics_testbed3d;

use na::{Isometry3, Point3, Vector3};
use ncollide::shape::{Cuboid, ShapeHandle};
use nphysics3d::world::World;
use nphysics3d::object::BodyHandle;
use nphysics3d::volumetric::Volumetric;
use nphysics_testbed3d::Testbed;

const COLLIDER_MARGIN: f32 = 0.01;

fn main() {
    /*
     * World
     */
    let mut world = World::new();
    world.set_gravity(Vector3::new(0.0, -9.81, 0.0));
    // world.set_max_velocity_iterations(40);
    world.set_erp(0.0);
    world.set_warmstart_factor(1.0);
    // world.set_max_position_iterations(3);

    /*
     * Ground.
     */
    let ground_size = 50.0;
    let ground_shape =
        ShapeHandle::new(Cuboid::new(Vector3::repeat(ground_size - COLLIDER_MARGIN)));
    let ground_pos = Isometry3::new(Vector3::y() * -ground_size, na::zero());
    world.add_collider(
        COLLIDER_MARGIN,
        ground_shape,
        BodyHandle::ground(),
        ground_pos,
    );

    /*
     * Create the boxes
     */
    let num = 8;
    let rad = 0.1;
    let shift = rad * 2.0;
    let centerx = shift * (num as f32) / 2.0;
    let centery = shift / 2.0;
    let centerz = shift * (num as f32) / 2.0;

    let geom = ShapeHandle::new(Cuboid::new(Vector3::repeat(rad - COLLIDER_MARGIN)));
    let inertia = geom.inertia(1.0);

    for i in 0usize..num {
        for j in 0usize..num {
            for k in 0usize..num {
                let x = i as f32 * shift - centerx;
                let y = j as f32 * shift + centery + 0.3;
                let z = k as f32 * shift - centerz;

                /*
                 * Create the rigid body.
                 */
                let pos = Isometry3::new(Vector3::new(x, y, z), na::zero());
                let handle = world.add_rigid_body(pos, inertia);
                world
                    .rigid_body_mut(handle)
                    .unwrap()
                    .activation_status_mut()
                    .set_deactivation_threshold(None);

                /*
                 * Create the collider.
                 */
                world.add_collider(COLLIDER_MARGIN, geom.clone(), handle, Isometry3::identity());
            }
        }
    }

    /*
     * Set up the testbed.
     */
    let mut testbed = Testbed::new(world);
    testbed.look_at(Point3::new(-10.0, 10.0, -10.0), Point3::origin());
    testbed.run();
}
