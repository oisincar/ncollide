use na::RealField;

use crate::math::{Point, Vector};
use crate::query::{self, Ray};
use crate::shape::Ball;

/// Time Of Impact of two balls under translational movement.
#[inline]
pub fn time_of_impact_ball_ball<N: RealField>(
    center1: &Point<N>,
    vel1: &Vector<N>,
    b1: &Ball<N>,
    center2: &Point<N>,
    vel2: &Vector<N>,
    b2: &Ball<N>,
    distance: N,
) -> Option<N>
{
    let vel = *vel1 - *vel2;
    let radius = b1.radius() + b2.radius() + distance;
    let center = *center1 + (-center2.coords);

    query::ray_toi_with_ball(&center, radius, &Ray::new(Point::origin(), -vel), true).1
}
