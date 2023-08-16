#[derive(Clone, Debug, PartialEq)]
pub enum WeaponKind {
    Sword,
    Axe,
    Bow,
    CrossBow,
    Spear,
}

/// A component that represents the damage that lasts for a certain amount of "time".
#[derive(Component, Debug, Clone)]
pub struct DamageOverTime {
    pub value: i32,
    pub duration: i32,
}

pub enum DamageKind {
    Physical,
    Fire,
    Ice,
    Poison,
}

#[derive(Component, Debug, Clone)]
pub struct AttackKind {}

#[derive(Component, Debug, Clone)]
pub struct Weapon {
    name: String,
    description: String,
    pub kind: WeaponKind,
    weight: i32,
}

// pub fn create_weapon() -> Entity {
//     let mut weapon = Entity::new();
//     weapon.add_component(Weapon::new(WeaponKind::Sword));
//     weapon.add_component(Name::new("Sword".to_string()));
//     weapon.add_component(Description::new("A sword".to_string()));
//     weapon.add_component(Weight::new(1));
//     weapon.add_component(Attack::new(1));
//     weapon.add_component(Defense::new(0));
//     weapon.add_component(Price::new(1));
//     weapon.add_component(Icon::new("sword".to_string()));
//     weapon
// }
