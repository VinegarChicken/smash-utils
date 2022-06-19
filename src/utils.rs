use smash::{
    app::{
        lua_bind,
        lua_bind::{StatusModule::*, *},
        sv_animcmd::{frame, wait},
        BattleObjectModuleAccessor,
    },
    hash40,
    lib::{lua_const::*, L2CValue},
    lua2cpp::{L2CAgentBase, L2CFighterCommon},
    phx::{Hash40, Vector3f},
    *,
};
use smash::app::{BattleObject, FighterEntryID};

#[skyline::from_offset(0x3ac540)]
pub fn get_battle_object_from_id(id: u32) -> *mut BattleObject;

extern "C" {
    #[link_name = "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E"]
    pub static FIGHTER_MANAGER: *mut smash::app::FighterManager;
}

pub fn is_grounded(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool {
    let situation_kind = unsafe { StatusModule::situation_kind(module_accessor) as i32 };
    situation_kind == SITUATION_KIND_GROUND
}

pub unsafe fn set_position_lock(entry_id: i32) {
    FighterManager::set_position_lock(FIGHTER_MANAGER, FighterEntryID(entry_id), true);
}

pub unsafe fn unset_position_lock(entry_id: i32) {
    FighterManager::set_position_lock(FIGHTER_MANAGER, FighterEntryID(entry_id), false);
}

pub fn get_module_accessor_by_entry_id(
    entry_id: i32,
) -> *mut smash::app::BattleObjectModuleAccessor {
    unsafe {
        &mut *smash::app::sv_battle_object::module_accessor(
            smash::app::Fighter::get_id_from_entry_id(entry_id),
        )
    }
}

pub unsafe fn change_motion(module_accessor: *mut BattleObjectModuleAccessor, anim: &str) {
    MotionModule::change_motion(
        module_accessor,
        Hash40::new(anim),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false,
    );
}

pub unsafe fn change_motion_inherit(module_accessor: *mut BattleObjectModuleAccessor, anim: &str) {
    MotionModule::change_motion_inherit_frame(
        module_accessor,
        smash::phx::Hash40::new(anim),
        0.0,
        1.0,
        0.0,
        false,
        false
    );
}

pub unsafe fn get_entry_id(module_accessor: *mut BattleObjectModuleAccessor) -> usize {
    WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize
}

pub unsafe fn disable_gravity(module_accessor: *mut BattleObjectModuleAccessor) {
    KineticModule::unable_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
}

pub unsafe fn enable_gravity(module_accessor: *mut BattleObjectModuleAccessor) {
    KineticModule::enable_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
}
