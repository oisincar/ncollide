use bounding_volume::{HasBoundingVolume, AABB};
use math::Isometry;
use na::{self, Real};
use shape::Compound;
use utils::IsometryOps;

impl<N: Real> HasBoundingVolume<N, AABB<N>> for Compound<N> {
    #[inline]
    fn bounding_volume(&self, m: &Isometry<N>) -> AABB<N> {
        let bv = self.aabb();
        bv.transform_by(m)
    }
}