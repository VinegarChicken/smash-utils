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

extern "C" {
    #[link_name = "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E"]
    pub static FIGHTER_MANAGER: *mut smash::app::FighterManager;
}