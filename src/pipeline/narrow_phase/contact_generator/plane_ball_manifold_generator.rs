use bounding_volume::ConicalApproximation;
use math::{Isometry, Point};
use na::{self, Real};
use pipeline::narrow_phase::{ContactDispatcher, ContactManifoldGenerator};
use query::{Contact, ContactKinematic, ContactManifold, ContactPrediction, NeighborhoodGeometry};
use shape::{Ball, FeatureId, Plane, Shape};
use utils::{IdAllocator, IsometryOps};

/// Collision detector between g1 plane and g1 shape implementing the `SupportMap` trait.
#[derive(Clone)]
pub struct PlaneBallManifoldGenerator<N: Real> {
    flip: bool,
    manifold: ContactManifold<N>,
}

impl<N: Real> PlaneBallManifoldGenerator<N> {
    /// Creates g1 new persistent collision detector between g1 plane and g1 shape with g1 support
    /// mapping function.
    #[inline]
    pub fn new(flip: bool) -> PlaneBallManifoldGenerator<N> {
        PlaneBallManifoldGenerator {
            flip,
            manifold: ContactManifold::new(),
        }
    }

    #[inline]
    fn do_update_to(
        m1: &Isometry<N>,
        g1: &Shape<N>,
        m2: &Isometry<N>,
        g2: &Shape<N>,
        prediction: &ContactPrediction<N>,
        id_alloc: &mut IdAllocator,
        manifold: &mut ContactManifold<N>,
        flip: bool,
    ) -> bool {
        if let (Some(plane), Some(ball)) = (g1.as_shape::<Plane<N>>(), g2.as_shape::<Ball<N>>()) {
            let plane_normal = m1 * plane.normal();
            let plane_center = Point::from_coordinates(m1.translation.vector);

            let ball_center = Point::from_coordinates(m2.translation.vector);
            let dist = na::dot(&(ball_center - plane_center), plane_normal.as_ref());
            let depth = -dist + ball.radius();

            if depth > -prediction.linear() {
                let world1 = ball_center + *plane_normal * (-dist);
                let world2 = ball_center + *plane_normal * (-ball.radius());

                let local1 = m1.inverse_transform_point(&world1);
                let local2 = Point::origin();

                let f1 = FeatureId::Face(0);
                let f2 = FeatureId::Face(0);
                let mut kinematic = ContactKinematic::new();
                let contact;

                let approx_ball = NeighborhoodGeometry::Point;
                let approx_plane = NeighborhoodGeometry::Plane(*plane.normal());

                if !flip {
                    contact = Contact::new(world1, world2, plane_normal, depth);
                    kinematic.set_approx1(f1, local1, approx_plane);
                    kinematic.set_approx2(f2, local2, approx_ball);
                    kinematic.set_dilation2(ball.radius());
                } else {
                    contact = Contact::new(world2, world1, -plane_normal, depth);
                    kinematic.set_approx1(f2, local2, approx_ball);
                    kinematic.set_dilation1(ball.radius());
                    kinematic.set_approx2(f1, local1, approx_plane);
                }

                let _ = manifold.push(contact, Point::origin(), kinematic, id_alloc);
            }

            true
        } else {
            false
        }
    }
}

impl<N: Real> ContactManifoldGenerator<N> for PlaneBallManifoldGenerator<N> {
    #[inline]
    fn generate_contacts(
        &mut self,
        _: &ContactDispatcher<N>,
        m1: &Isometry<N>,
        g1: &Shape<N>,
        fmap1: Option<&Fn(FeatureId) -> FeatureId>,
        m2: &Isometry<N>,
        g2: &Shape<N>,
        fmap2: Option<&Fn(FeatureId) -> FeatureId>,
        prediction: &ContactPrediction<N>,
        id_alloc: &mut IdAllocator,
        manifold: &mut ContactManifold<N>,
    ) -> bool {
        /*
        if !self.flip {
            Self::do_update_to(
                m1,
                g1,
                m2,
                g2,
                prediction,
                id_alloc,
                &mut self.manifold,
                false,
            )
        } else {
            Self::do_update_to(
                m2,
                g2,
                m1,
                g1,
                prediction,
                id_alloc,
                &mut self.manifold,
                true,
            )
        }*/
        unimplemented!()
    }
}
