use cgmath::{InnerSpace, Vector2};

use specs::{
    Builder, Component, Join, ParJoin, Read, ReadStorage, System, VecStorage, World, WorldExt,
    WriteStorage,
};

use crate::engine_interaction::DeltaTime;
use crate::interaction::{Movable, Selectable};
use crate::physics::add_shape;
use crate::physics::physics_components::{Drag, Kinematics, Transform};
use crate::rendering::meshrender_component::{CircleRender, MeshRenderComponent};
use cgmath::num_traits::zero;
use ncollide2d::shape::Ball;
use specs::prelude::ParallelIterator;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Human {
    objective: Vector2<f32>,
}

impl Human {
    fn calc_acceleration(
        &self,
        //position: &transform,
        _kin: &Kinematics,
        //others: &[(&transform, &Human)],
    ) -> Vector2<f32> {
        zero()
    }
}

pub struct HumanUpdate;

impl<'a> System<'a> for HumanUpdate {
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, Kinematics>,
        ReadStorage<'a, Human>,
    );

    fn run(&mut self, (delta, transforms, mut kinematics, humans): Self::SystemData) {
        let _delta = delta.0;

        let _xx: Vec<(&Transform, &Human)> = (&transforms, &humans).join().collect();

        (&transforms, &mut kinematics, &humans)
            .par_join()
            .for_each(|(t, k, h)| {
                if (h.objective - t.position()).magnitude2() < 1.0 {
                    k.velocity = [0.0, 0.0].into();
                    return;
                }

                let acc = h.calc_acceleration(&k);
                k.acceleration += acc;
            })
    }
}

pub fn setup(world: &mut World) {
    const SCALE: f32 = 100.0;

    for _ in 0..1 {
        let size = 1.0;

        let x: f32 = rand::random::<f32>() * SCALE;
        let y: f32 = rand::random::<f32>() * SCALE;

        let eb = world
            .create_entity()
            .with(MeshRenderComponent::from(CircleRender {
                radius: size,
                ..Default::default()
            }))
            .with(Transform::new((x, y)))
            .with(Kinematics::from_mass(70.0))
            .with(Drag::default())
            .with(Human {
                objective: [SCALE * 5.0 - x, y].into(),
            })
            .with(Selectable)
            .with(Movable);

        let e = eb.build();
        //let shape = Ball::new(size);
        let shape = Ball::new(size);

        add_shape(world, e, shape);
    }
}
