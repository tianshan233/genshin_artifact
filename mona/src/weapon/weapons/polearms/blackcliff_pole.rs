use crate::attribute::{Attribute, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::WeaponConfig;

pub const BLACKCLIFF_POLE_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Polearm,
    weapon_sub_stat: WeaponSubStatFamily::CriticalDamage120,
    weapon_base: WeaponBaseATKFamily::ATK510,
    star: 4
};

pub struct BlackcliffPoleEffect {
    stack: f64
}

impl BlackcliffPoleEffect {
    pub fn new(config: &WeaponConfig) -> BlackcliffPoleEffect {
        match *config {
            WeaponConfig::BlackcliffPole { stack } => BlackcliffPoleEffect {
                stack
            },
            _ => BlackcliffPoleEffect {
                stack: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for BlackcliffPoleEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.03 + 0.09) * self.stack;
        attribute.add_atk_percentage("黑岩刺枪被动等效", value);
    }
}