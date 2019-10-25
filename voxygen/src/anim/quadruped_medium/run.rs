use super::{
    super::{Animation, SkeletonAttr},
    QuadrupedMediumSkeleton,
};
use std::{f32::consts::PI, ops::Mul};
use vek::*;

pub struct RunAnimation;

impl Animation for RunAnimation {
    type Skeleton = QuadrupedMediumSkeleton;
    type Dependency = (f32, f64);

    fn update_skeleton(
        skeleton: &Self::Skeleton,
        (_velocity, global_time): Self::Dependency,
        anim_time: f64,
        _rate: &mut f32,
        _skeleton_attr: &SkeletonAttr,
    ) -> Self::Skeleton {
        let mut next = (*skeleton).clone();

        let wave = (anim_time as f32 * 14.0).sin();
        let wave_slow = (anim_time as f32 * 3.5 + PI).sin();
        let wave_slow_cos = (anim_time as f32 * 3.5 + PI).cos();
        let wave_quick = (anim_time as f32 * 18.0).sin();
        let wave_med = (anim_time as f32 * 12.0).sin();
        let wave_med_cos = (anim_time as f32 * 12.0).cos();
        let wave_quick_cos = (anim_time as f32 * 18.0).cos();

        let wolf_look = Vec2::new(
            ((global_time + anim_time) as f32 / 4.0)
                .floor()
                .mul(7331.0)
                .sin()
                * 0.25,
            ((global_time + anim_time) as f32 / 4.0)
                .floor()
                .mul(1337.0)
                .sin()
                * 0.125,
        );

        next.head_upper.offset =
            Vec3::new(0.0, 9.5 + wave_quick_cos * 2.0, 15.0 + wave_med * 3.0) / 11.0;
        next.head_upper.ori = Quaternion::rotation_x(-0.12 + wave_quick_cos * 0.12 + wolf_look.y)
            * Quaternion::rotation_z(wolf_look.x);
        next.head_upper.scale = Vec3::one() / 10.88;

        next.jaw.offset = Vec3::new(0.0, 4.5, 2.0 + wave_slow_cos * 1.0);
        next.jaw.ori = Quaternion::rotation_x(wave_slow * 0.05);
        next.jaw.scale = Vec3::one() * 1.01;

        next.head_lower.offset = Vec3::new(0.0, 3.1, -4.5 + wave_med * 1.0);
        next.head_lower.ori = Quaternion::rotation_z(0.0);
        next.head_lower.scale = Vec3::one() * 0.98;

        next.tail.offset = Vec3::new(0.0, -12.0, 10.0) / 11.0;
        next.tail.ori = Quaternion::rotation_x(wave_quick * 0.18);
        next.tail.scale = Vec3::one() / 11.0;

        next.torso_back.offset =
            Vec3::new(0.0, -9.5 + wave_quick_cos * 2.2, 13.0 + wave_med * 2.8) / 11.0;
        next.torso_back.ori = Quaternion::rotation_x(-0.15 + wave_med_cos * 0.14);
        next.torso_back.scale = Vec3::one() / 11.0;

        next.torso_mid.offset =
            Vec3::new(0.0, 0.0 + wave_quick_cos * 2.2, 14.0 + wave_med * 3.2) / 11.0;
        next.torso_mid.ori = Quaternion::rotation_x(-0.15 + wave_med_cos * 0.12);
        next.torso_mid.scale = Vec3::one() / 10.5;

        next.ears.offset = Vec3::new(0.0, 0.75 + wave * 0.4, 6.25);
        next.ears.ori = Quaternion::rotation_x(wave * 0.2);
        next.ears.scale = Vec3::one() * 1.05;

        next.foot_lf.offset =
            Vec3::new(-5.0, 5.0 + wave_quick * 3.0, 7.0 + wave_quick_cos * 4.0) / 11.0;
        next.foot_lf.ori = Quaternion::rotation_x(0.0 + wave_quick * 0.8);
        next.foot_lf.scale = Vec3::one() / 11.0;

        next.foot_rf.offset =
            Vec3::new(5.0, 5.0 - wave_quick_cos * 3.0, 7.0 + wave_quick * 4.0) / 11.0;
        next.foot_rf.ori = Quaternion::rotation_x(0.0 - wave_quick_cos * 0.8);
        next.foot_rf.scale = Vec3::one() / 11.0;

        next.foot_lb.offset =
            Vec3::new(-5.0, -10.0 - wave_quick_cos * 3.0, 7.0 + wave_quick * 4.0) / 11.0;
        next.foot_lb.ori = Quaternion::rotation_x(0.0 - wave_quick_cos * 0.8);
        next.foot_lb.scale = Vec3::one() / 11.0;

        next.foot_rb.offset =
            Vec3::new(5.0, -10.0 + wave_quick * 3.0, 7.0 + wave_quick_cos * 4.0) / 11.0;
        next.foot_rb.ori = Quaternion::rotation_x(0.0 + wave_quick * 0.8);
        next.foot_rb.scale = Vec3::one() / 11.0;

        next
    }
}
