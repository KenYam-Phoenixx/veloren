use super::{
    super::{Animation, vek::*},
    QuadrupedLowSkeleton, SkeletonAttr,
};
use common::states::utils::StageSection;

pub struct ShootAnimation;

impl Animation for ShootAnimation {
    type Dependency<'a> = (Option<&'a str>, f32, f32, Option<StageSection>);
    type Skeleton = QuadrupedLowSkeleton;

    #[cfg(feature = "use-dyn-lib")]
    const UPDATE_FN: &'static [u8] = b"quadruped_low_shoot\0";

    #[cfg_attr(feature = "be-dyn-lib", unsafe(export_name = "quadruped_low_shoot"))]
    fn update_skeleton_inner(
        skeleton: &Self::Skeleton,
        (ability_id, velocity, _global_time, stage_section): Self::Dependency<'_>,
        anim_time: f32,
        _rate: &mut f32,
        s_a: &SkeletonAttr,
    ) -> Self::Skeleton {
        let mut next = (*skeleton).clone();

        let (movement1, movement2, movement3) = match stage_section {
            Some(StageSection::Buildup) => (anim_time, 0.0, 0.0),
            Some(StageSection::Action) => (1.0, anim_time.powf(0.25), 0.0),
            Some(StageSection::Recover) => (1.0, 1.0, anim_time),
            _ => (0.0, 0.0, 0.0),
        };

        let twitch1 = (movement1 * 18.0).sin();
        match ability_id {
            Some("common.abilities.custom.dwarves.snaretongue.bombs") => {
                next.head_c_upper.position =
                    Vec3::new(0.0, s_a.head_upper.0, s_a.head_upper.1 - movement1 * 2.0);
                next.head_c_upper.orientation =
                    Quaternion::rotation_x((movement1 * 0.3 + movement2 * 0.2) * (1.0 - movement3))
                        * Quaternion::rotation_z((twitch1 * 0.1) * (1.0 - movement3));

                next.head_c_lower.position =
                    Vec3::new(0.0, s_a.head_lower.0, s_a.head_lower.1 - movement1 * 0.5);
                next.head_c_lower.orientation = Quaternion::rotation_x(
                    (twitch1 * 0.05 + movement1 * 0.3 + movement2 * 0.2) * (0.5 - movement3),
                );

                next.jaw_c.position = Vec3::new(0.0, s_a.jaw.0, s_a.jaw.1);
                next.jaw_c.orientation = Quaternion::rotation_x(movement1 * -0.5 + movement3 * 0.5);

                next.chest.position = Vec3::new(0.0, s_a.chest.0, s_a.chest.1 + movement1 * 0.5);
                next.chest.orientation = Quaternion::rotation_x(0.0 + movement1 * 0.1);

                next.tail_front.scale = Vec3::one() * 0.1;
                next.tail_rear.scale = Vec3::one() * 0.1;

                next.foot_fl.orientation = Quaternion::rotation_x(movement1 * -0.2);

                next.foot_fr.orientation = Quaternion::rotation_x(movement1 * -0.2);

                next.foot_bl.orientation = Quaternion::rotation_x(movement1 * -0.2);

                next.foot_br.orientation = Quaternion::rotation_x(movement1 * -0.2);
                if velocity < 0.5 {
                    next.foot_fl.position = Vec3::new(-s_a.feet_f.0, s_a.feet_f.1, s_a.feet_f.2);

                    next.foot_fr.position = Vec3::new(s_a.feet_f.0, s_a.feet_f.1, s_a.feet_f.2);

                    next.foot_bl.position = Vec3::new(-s_a.feet_b.0, s_a.feet_b.1, s_a.feet_b.2);

                    next.foot_br.position = Vec3::new(s_a.feet_b.0, s_a.feet_b.1, s_a.feet_b.2);
                };
            },
            Some("common.abilities.custom.dagon.dagonbombs") => {
                next.head_c_upper.position =
                    Vec3::new(0.0, s_a.head_upper.0, s_a.head_upper.1 - movement1 * 2.0);
                next.head_c_upper.orientation =
                    Quaternion::rotation_x((movement1 * 0.3 + movement2 * 0.2) * (1.0 - movement3))
                        * Quaternion::rotation_z((twitch1 * 0.1) * (1.0 - movement3));

                next.head_c_lower.position =
                    Vec3::new(0.0, s_a.head_lower.0, s_a.head_lower.1 - movement1 * 0.5);
                next.head_c_lower.orientation = Quaternion::rotation_x(
                    (twitch1 * 0.05 + movement1 * 0.3 + movement2 * 0.2) * (0.5 - movement3),
                );

                next.jaw_c.position = Vec3::new(0.0, s_a.jaw.0, s_a.jaw.1);
                next.jaw_c.orientation = Quaternion::rotation_x(movement1 * -0.5 + movement3 * 0.5);

                next.chest.position = Vec3::new(0.0, s_a.chest.0, s_a.chest.1 + movement1 * 0.5);
                next.chest.orientation = Quaternion::rotation_x(0.0 + movement1 * 0.1);

                next.tail_front.position =
                    Vec3::new(0.0, s_a.tail_front.0 + movement1 * 1.5, s_a.tail_front.1);
                next.tail_front.orientation =
                    Quaternion::rotation_x(movement1 * -0.4 + movement2 * 0.3);

                next.tail_rear.position =
                    Vec3::new(0.0, s_a.tail_rear.0 + movement1 * 1.5, s_a.tail_rear.1);
                next.tail_rear.orientation =
                    Quaternion::rotation_x(movement1 * -0.4 + movement2 * 0.3);
                next.foot_fl.orientation = Quaternion::rotation_x(movement1 * -0.2);

                next.foot_fr.orientation = Quaternion::rotation_x(movement1 * -0.2);

                next.foot_bl.orientation = Quaternion::rotation_x(movement1 * -0.2);

                next.foot_br.orientation = Quaternion::rotation_x(movement1 * -0.2);
                if velocity < 0.5 {
                    next.foot_fl.position = Vec3::new(-s_a.feet_f.0, s_a.feet_f.1, s_a.feet_f.2);

                    next.foot_fr.position = Vec3::new(s_a.feet_f.0, s_a.feet_f.1, s_a.feet_f.2);

                    next.foot_bl.position = Vec3::new(-s_a.feet_b.0, s_a.feet_b.1, s_a.feet_b.2);

                    next.foot_br.position = Vec3::new(s_a.feet_b.0, s_a.feet_b.1, s_a.feet_b.2);
                };
            },
            _ => {
                // Center head
                next.head_c_upper.position = Vec3::new(0.0, s_a.head_upper.0, s_a.head_upper.1);
                next.head_c_upper.orientation =
                    Quaternion::rotation_x((movement1 * 0.6 + movement2 * 0.7) * (1.0 - movement3))
                        * Quaternion::rotation_z((twitch1 * 0.1) * (1.0 - movement3));

                next.head_c_lower.position = Vec3::new(0.0, s_a.head_lower.0, s_a.head_lower.1);
                next.head_c_lower.orientation = Quaternion::rotation_x(
                    (twitch1 * 0.05 + movement1 * 0.3 + movement2 * 0.6) * (1.0 - movement3),
                );

                next.jaw_c.position = Vec3::new(0.0, s_a.jaw.0, s_a.jaw.1);
                next.jaw_c.orientation = Quaternion::rotation_x(movement1 * -0.5);

                // Left head
                next.head_l_upper.position = Vec3::new(
                    -s_a.side_head_upper.0,
                    s_a.side_head_upper.1,
                    s_a.side_head_upper.2,
                );
                next.head_l_upper.orientation =
                    Quaternion::rotation_x((movement1 * 0.6 + movement2 * 0.7) * (1.0 - movement3))
                        * Quaternion::rotation_z((twitch1 * 0.1) * (1.0 - movement3));

                next.head_l_lower.position = Vec3::new(
                    -s_a.side_head_lower.0,
                    s_a.side_head_lower.1,
                    s_a.side_head_lower.2,
                );
                next.head_l_lower.orientation = Quaternion::rotation_x(
                    (twitch1 * 0.05 + movement1 * 0.3 + movement2 * 0.6) * (1.0 - movement3),
                );

                next.jaw_l.position = Vec3::new(0.0, s_a.jaw.0, s_a.jaw.1);
                next.jaw_l.orientation = Quaternion::rotation_x(movement1 * -0.5);

                // Right head
                next.head_r_upper.position = Vec3::new(
                    s_a.side_head_upper.0,
                    s_a.side_head_upper.1,
                    s_a.side_head_upper.2,
                );
                next.head_r_upper.orientation =
                    Quaternion::rotation_x((movement1 * 0.6 + movement2 * 0.7) * (1.0 - movement3))
                        * Quaternion::rotation_z((twitch1 * 0.1) * (1.0 - movement3));

                next.head_r_lower.position = Vec3::new(
                    s_a.side_head_lower.0,
                    s_a.side_head_lower.1,
                    s_a.side_head_lower.2,
                );
                next.head_r_lower.orientation = Quaternion::rotation_x(
                    (twitch1 * 0.05 + movement1 * 0.3 + movement2 * 0.6) * (1.0 - movement3),
                );

                next.jaw_r.position = Vec3::new(0.0, s_a.jaw.0, s_a.jaw.1);
                next.jaw_r.orientation = Quaternion::rotation_x(movement1 * -0.5);

                next.chest.position = Vec3::new(0.0, s_a.chest.0, s_a.chest.1);
                next.chest.orientation = Quaternion::rotation_x(0.0);

                next.tail_front.position = Vec3::new(0.0, s_a.tail_front.0, s_a.tail_front.1);
                next.tail_front.orientation = Quaternion::rotation_x(movement1 * 0.15);

                next.tail_rear.position = Vec3::new(0.0, s_a.tail_rear.0, s_a.tail_rear.1);
                next.tail_rear.orientation = Quaternion::rotation_x(0.0);
                if velocity < 0.5 {
                    next.foot_fl.position = Vec3::new(-s_a.feet_f.0, s_a.feet_f.1, s_a.feet_f.2);

                    next.foot_fr.position = Vec3::new(s_a.feet_f.0, s_a.feet_f.1, s_a.feet_f.2);

                    next.foot_bl.position = Vec3::new(-s_a.feet_b.0, s_a.feet_b.1, s_a.feet_b.2);

                    next.foot_br.position = Vec3::new(s_a.feet_b.0, s_a.feet_b.1, s_a.feet_b.2);
                };
            },
        }
        next
    }
}
