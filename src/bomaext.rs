//much of this is from https://github.com/HDR-Development/HewDraw-Remix


#[repr(C)]
pub struct ModelColorType(pub i32);

use smash::app::{
    self,
    *,
    lua_bind::{*, FighterManager},
    FighterKineticEnergyMotion,
    FighterKineticEnergyController,
};
use smash::lua2cpp::*;
use smash::lib::{
    *,
    lua_const::*
};
use smash::phx::*;
use crate::cmdflag::*;

#[skyline::from_offset(0x3ac540)]
pub fn get_battle_object_from_id(id: u32) -> *mut BattleObject;

extern "C" {
    #[link_name = "\u{1}_ZN3app8lua_bind31ModelModule__set_color_rgb_implEPNS_26BattleObjectModuleAccessorEfffNS_16MODEL_COLOR_TYPEE"]
    pub fn set_color_rgb(
        arg1: *mut BattleObjectModuleAccessor,
        arg2: f32,
        arg3: f32,
        arg4: f32,
        arg5: ModelColorType,
    );
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AerialKind {
    Nair,
    Fair,
    Bair,
    Uair,
    Dair
}
pub trait BomaExt{
    // INPUTS
    unsafe fn is_cat_flag<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool;
    unsafe fn is_cat_flag_all<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool;
    unsafe fn is_pad_flag(&mut self, pad_flag: PadFlag) -> bool;
    unsafe fn is_button_on(&mut self, buttons: Buttons) -> bool;
    unsafe fn is_button_off(&mut self, buttons: Buttons) -> bool;
    unsafe fn is_button_trigger(&mut self, buttons: Buttons) -> bool;
    unsafe fn is_button_release(&mut self, buttons: Buttons) -> bool;
    unsafe fn was_prev_button_on(&mut self, buttons: Buttons) -> bool;
    unsafe fn was_prev_button_off(&mut self, buttons: Buttons) -> bool;
    unsafe fn stick_x(&mut self) -> f32;
    unsafe fn stick_y(&mut self) -> f32;
    unsafe fn prev_stick_x(&mut self) -> f32;
    unsafe fn prev_stick_y(&mut self) -> f32;
    unsafe fn is_flick_y(&mut self, sensitivity: f32) -> bool;
    unsafe fn is_input_jump(&mut self) -> bool;
    unsafe fn get_aerial(&mut self) -> Option<AerialKind>;
    unsafe fn set_joint_rotate(&mut self, bone_name: &str, rotation: Vector3f);
    unsafe fn object(&mut self) -> *mut BattleObject;
    unsafe fn is_grounded(&mut self) -> bool;
    unsafe fn change_motion(&mut self, motion_kind: Hash40, inherit: bool);
    unsafe fn set_position_lock(&mut self);
    unsafe fn unset_position_lock(&mut self);
    unsafe fn set_position(&mut self, pos: &Vector3f);
    unsafe fn is_damage_check(&mut self, is_prev: bool) -> bool;
    unsafe fn situation_kind(&mut self) -> i32;
    unsafe fn status_kind(&mut self) -> i32;
    unsafe fn set_gravity(&mut self, disable: bool);
    /// returns whether or not the stick x is pointed in the "forwards" direction for
    /// a character
    unsafe fn is_stick_forward(&mut self) -> bool;
    unsafe fn get_entry_id(&mut self) -> usize;
    unsafe fn enable_jump(&mut self);

    /// returns whether or not the stick x is pointed in the "backwards" direction for
    /// a character
    unsafe fn is_stick_backward(&mut self) -> bool;

    unsafe fn set_color_rgb(&mut self, r: f32, g: f32, b: f32, MODEL_COLOR_TYPE);

    // STATE
    unsafe fn is_status(&mut self, kind: i32) -> bool;
    unsafe fn is_status_one_of(&mut self, kinds: &[i32]) -> bool;
    unsafe fn is_prev_status(&mut self, kind: i32) -> bool;
    unsafe fn is_prev_status_one_of(&mut self, kinds: &[i32]) -> bool;
    unsafe fn is_situation(&mut self, kind: i32) -> bool;
    unsafe fn is_prev_situation(&mut self, kind: i32) -> bool;
    unsafe fn is_motion(&mut self, motion: Hash40) -> bool;
    unsafe fn is_motion_one_of(&mut self, motions: &[Hash40]) -> bool;
    unsafe fn status(&mut self) -> i32;
    unsafe fn enable_cancel(&mut self);
    /// gets the number of jumps that have been used
    unsafe fn get_num_used_jumps(&mut self) -> i32;

    /// gets the max allowed number of jumps for this character
    unsafe fn get_jump_count_max(&mut self) -> i32;
    unsafe fn motion_frame(&mut self) -> f32;
    unsafe fn set_rate(&mut self, motion_rate: f32);
    unsafe fn set_scale(&mut self, scale: f32);
    unsafe fn set_joint_scale(&mut self, joint: Hash40, scale: *const Vector3f);
    unsafe fn is_in_hitlag(&mut self) -> bool;

    unsafe fn get_owner_boma(&mut self) -> *mut BattleObjectModuleAccessor;

    unsafe fn change_status(&mut self, kind: i32, repeat: bool) -> i32;

    // INSTANCE
    unsafe fn is_fighter(&mut self) -> bool;
    unsafe fn is_weapon(&mut self) -> bool;
    unsafe fn kind(&mut self) -> i32;

    // WORK
    unsafe fn get_int(&mut self, what: i32) -> i32;
    unsafe fn get_float(&mut self, what: i32) -> f32;
    unsafe fn get_int64(&mut self, what: i32) -> u64;
    unsafe fn is_flag(&mut self, what: i32) -> bool;
    unsafe fn set_int(&mut self, value: i32, what: i32);
    unsafe fn set_float(&mut self, value: f32, what: i32);
    unsafe fn set_int64(&mut self, value: i64, what: i32);
    unsafe fn on_flag(&mut self, what: i32);
    unsafe fn off_flag(&mut self, what: i32);
    unsafe fn get_param_int(&mut self, obj: &str, field: &str) -> i32;
    unsafe fn get_param_float(&mut self, obj: &str, field: &str) -> f32;
    unsafe fn get_param_int64(&mut self, obj: &str, field: &str) -> u64;

    // ENERGY
    unsafe fn get_motion_energy(&mut self) -> &mut FighterKineticEnergyMotion;
    unsafe fn get_controller_energy(&mut self) -> &mut FighterKineticEnergyController;

}


impl BomaExt for smash::app::BattleObjectModuleAccessor {
    unsafe fn is_cat_flag<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool {
        let cat = fighter_pad_cmd_flag.into();
        match cat {
            CommandCat::Cat1(cat) => Cat1::new(self).intersects(cat),
            CommandCat::Cat2(cat) => Cat2::new(self).intersects(cat),
            CommandCat::Cat3(cat) => Cat3::new(self).intersects(cat),
            CommandCat::Cat4(cat) => Cat4::new(self).intersects(cat),
        }
    }

    unsafe fn is_cat_flag_all<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool {
        let cat = fighter_pad_cmd_flag.into();
        match cat {
            CommandCat::Cat1(cat) => Cat1::new(self).contains(cat),
            CommandCat::Cat2(cat) => Cat2::new(self).contains(cat),
            CommandCat::Cat3(cat) => Cat3::new(self).contains(cat),
            CommandCat::Cat4(cat) => Cat4::new(self).contains(cat),
        }
    }
    unsafe fn enable_cancel(&mut self) {
        CancelModule::enable_cancel(self);
    }
    unsafe fn is_pad_flag(&mut self, pad_flag: PadFlag) -> bool {
        PadFlag::from_bits_unchecked(ControlModule::get_pad_flag(self)).intersects(pad_flag)
    }

    unsafe fn is_button_on(&mut self, buttons: Buttons) -> bool {
        Buttons::from_bits_unchecked(ControlModule::get_button(self)).intersects(buttons)
    }

    unsafe fn is_button_off(&mut self, buttons: Buttons) -> bool {
        !self.is_button_on(buttons)
    }

    unsafe fn is_button_trigger(&mut self, buttons: Buttons) -> bool {
        Buttons::from_bits_unchecked(ControlModule::get_trigger(self)).intersects(buttons)
    }

    unsafe fn is_button_release(&mut self, buttons: Buttons) -> bool {
        Buttons::from_bits_unchecked(ControlModule::get_release(self)).intersects(buttons)
    }

    unsafe fn was_prev_button_on(&mut self, buttons: Buttons) -> bool {
        Buttons::from_bits_unchecked(ControlModule::get_button_prev(self)).intersects(buttons)
    }

    unsafe fn was_prev_button_off(&mut self, buttons: Buttons) -> bool {
        !self.was_prev_button_on(buttons)
    }

    unsafe fn stick_x(&mut self) -> f32 {
        return ControlModule::get_stick_x(self);
    }

    unsafe fn stick_y(&mut self) -> f32 {
        return ControlModule::get_stick_y(self);
    }

    unsafe fn prev_stick_x(&mut self) -> f32 {
        return ControlModule::get_stick_prev_x(self);
    }

    unsafe fn prev_stick_y(&mut self) -> f32 {
        return ControlModule::get_stick_prev_y(self);
    }

    // TODO: Reimplement this check
    unsafe fn is_flick_y(&mut self, sensitivity: f32) -> bool {
        let stick = self.stick_y();
        let p_stick = self.prev_stick_y();

        if sensitivity < 0.0 && stick < sensitivity && (stick < p_stick || self.is_cat_flag(Cat2::FallJump)) {
            return true;
        }

        if sensitivity > 0.0 && stick > sensitivity && (stick > p_stick || self.is_cat_flag(Cat2::FallJump)) {
            return true;
        }

        return false;
    }
    unsafe fn enable_jump(&mut self) {
        if self.is_input_jump() {
            if self.situation_kind() == *SITUATION_KIND_GROUND {
                self.change_status(*FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
            }
            else if self.get_num_used_jumps() < self.get_jump_count_max() {
                self.change_status(*FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
            }
        }
    }
    unsafe fn is_input_jump(&mut self) -> bool {
        if self.is_cat_flag(Cat1::Jump) && ControlModule::is_enable_flick_jump(self) {
            WorkModule::set_int(self, 1, *FIGHTER_INSTANCE_WORK_ID_INT_STICK_JUMP_COMMAND_LIFE);
            return true;
        }

        return self.is_cat_flag(Cat1::JumpButton);
    }

    unsafe fn get_aerial(&mut self) -> Option<AerialKind> {
        if self.is_cat_flag(Cat1::AttackHi3 | Cat1::AttackHi4) {
            Some(AerialKind::Uair)
        } else if self.is_cat_flag(Cat1::AttackLw3 | Cat1::AttackLw4) {
            Some(AerialKind::Dair)
        } else if self.is_cat_flag(Cat1::AttackS3 | Cat1::AttackS4) {
            if self.is_stick_backward() {
                Some(AerialKind::Bair)
            } else {
                Some(AerialKind::Fair)
            }
        } else if self.is_cat_flag(Cat1::AttackN | Cat1::AttackAirN) {
            Some(AerialKind::Nair)
        } else {
            None
        }
    }
    unsafe fn situation_kind(&mut self) -> i32 {
        StatusModule::situation_kind(self)
    }
    unsafe fn set_joint_rotate(&mut self, bone_name: &str, rotation: Vector3f) {
        ModelModule::set_joint_rotate(self, Hash40::new(&bone_name), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8})
    }

    unsafe fn object(&mut self) -> *mut BattleObject {
        get_battle_object_from_id(self.battle_object_id)
    }

    unsafe fn is_grounded(&mut self) -> bool {
        StatusModule::situation_kind(self) == *SITUATION_KIND_GROUND
    }

    unsafe fn change_motion(&mut self, motion_kind: Hash40, inherit: bool) {
        if inherit{
            MotionModule::change_motion_inherit_frame(
                self,
                motion_kind,
                0.0,
                1.0,
                0.0,
                false,
                false
            );
        }
        else{
            MotionModule::change_motion(
                self,
                motion_kind,
                0.0,
                1.0,
                false,
                0.0,
                false,
                false,
            );
        }
    }

    unsafe fn set_position_lock(&mut self) {
        FighterManager::set_position_lock(FIGHTER_MANAGER, FighterEntryID(self.get_entry_id() as i32), true);
    }

    unsafe fn unset_position_lock(&mut self) {
        FighterManager::set_position_lock(FIGHTER_MANAGER, FighterEntryID(self.get_entry_id() as i32), false);
    }

    unsafe fn set_position(&mut self, pos: &Vector3f) {
        PostureModule::set_pos(self, *&pos);
    }

    unsafe fn is_damage_check(&mut self, is_prev : bool) -> bool {
        let status : i32;
        if is_prev {
            status = StatusModule::prev_status_kind(self, 0);
        }
        else {
            status = StatusModule::status_kind(self);
        }
        if FighterStopModuleImpl::is_damage_stop(self) || CaptureModule::is_capture(self)
            || WorkModule::is_flag(self, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAPTURE_YOSHI)
            || WorkModule::is_flag(self, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND)
            || WorkModule::is_flag(self, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR)
            || [
            *FIGHTER_STATUS_KIND_AIR_LASSO,
            *FIGHTER_STATUS_KIND_BIND,
            *FIGHTER_STATUS_KIND_BURY,
            *FIGHTER_STATUS_KIND_BURY_WAIT,
            *FIGHTER_STATUS_KIND_CAPTURE_BEETLE,
            *FIGHTER_STATUS_KIND_CAPTURE_CUT,
            *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
            *FIGHTER_STATUS_KIND_CAPTURE_DRIVER,
            *FIGHTER_STATUS_KIND_CAPTURE_ITEM,
            *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE,
            *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND,
            *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD,
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED_FISHINGROD,
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL,
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI,
            *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
            *FIGHTER_STATUS_KIND_CAPTURE_YOSHI,
            *FIGHTER_STATUS_KIND_CATCHED_AIR_END_GANON,
            *FIGHTER_STATUS_KIND_CATCHED_AIR_FALL_GANON,
            *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON,
            *FIGHTER_STATUS_KIND_CATCHED_GANON,
            *FIGHTER_STATUS_KIND_CATCHED_PICKEL_TROLLEY,
            *FIGHTER_STATUS_KIND_CATCHED_REFLET,
            *FIGHTER_STATUS_KIND_CATCHED_RIDLEY,
            *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED,
            *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN,
            *FIGHTER_STATUS_KIND_CLUNG_DAMAGE_DIDDY,
            *FIGHTER_STATUS_KIND_CLUNG_DIDDY,
            *FIGHTER_STATUS_KIND_CLUNG_GANON,
            *FIGHTER_STATUS_KIND_CLUNG_THROWN_BLANK_DIDDY,
            *FIGHTER_STATUS_KIND_CLUNG_THROWN_DIDDY,
            *FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_SLEEP,
            *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_END,
            *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL,
            *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START,
            *FIGHTER_STATUS_KIND_DAMAGE_SONG,
            *FIGHTER_STATUS_KIND_DAMAGE_SONG_END,
            *FIGHTER_STATUS_KIND_DAMAGE_SONG_FALL,
            *FIGHTER_STATUS_KIND_DAMAGE_SONG_START,
            *FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_DOWN,
            *FIGHTER_STATUS_KIND_DOWN_DAMAGE,
            *FIGHTER_STATUS_KIND_DOWN_EAT,
            *FIGHTER_STATUS_KIND_DOWN_SPOT,
            *FIGHTER_STATUS_KIND_DOWN_STAND,
            *FIGHTER_STATUS_KIND_DOWN_STAND_FB,
            *FIGHTER_STATUS_KIND_DOWN_WAIT,
            *FIGHTER_STATUS_KIND_FINAL,
            *FIGHTER_STATUS_KIND_FURAFURA,
            *FIGHTER_STATUS_KIND_FURAFURA_END,
            *FIGHTER_STATUS_KIND_FURAFURA_STAND,
            *FIGHTER_STATUS_KIND_GUARD_DAMAGE,
            *FIGHTER_STATUS_KIND_ICE,
            *FIGHTER_STATUS_KIND_KOOPA_DIVED,
            *FIGHTER_STATUS_KIND_LAY_DOWN,
            *FIGHTER_STATUS_KIND_MEWTWO_THROWN,
            *FIGHTER_STATUS_KIND_MISS_FOOT,
            *FIGHTER_STATUS_KIND_PASSIVE,
            *FIGHTER_STATUS_KIND_PASSIVE_CEIL,
            *FIGHTER_STATUS_KIND_PASSIVE_FB,
            *FIGHTER_STATUS_KIND_PASSIVE_WALL,
            *FIGHTER_STATUS_KIND_REBIRTH,
            *FIGHTER_STATUS_KIND_SAVING_DAMAGE,
            *FIGHTER_STATUS_KIND_SAVING_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_SAVING_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_SHIELD_BREAK_DOWN,
            *FIGHTER_STATUS_KIND_SHIELD_BREAK_FALL,
            *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY,
            *FIGHTER_STATUS_KIND_SLEEP,
            *FIGHTER_STATUS_KIND_SLIP,
            *FIGHTER_STATUS_KIND_SLIP_DAMAGE,
            *FIGHTER_STATUS_KIND_SLIP_WAIT,
            *FIGHTER_STATUS_KIND_SLIP_STAND,
            *FIGHTER_STATUS_KIND_SLIP_STAND_B,
            *FIGHTER_STATUS_KIND_SLIP_STAND_F,
            *FIGHTER_STATUS_KIND_SLIP_STAND_ATTACK,
            *FIGHTER_STATUS_KIND_STABBED_DAMAGE,
            *FIGHTER_STATUS_KIND_STABBED_RIDLEY,
            *FIGHTER_STATUS_KIND_SWALLOWED,
            *FIGHTER_STATUS_KIND_THROWN,
        ].contains(&status) {
            true
        }
        else {
            false
        }
    }

    /// returns whether or not the stick x is pointed in the "forwards" direction for
    /// a character
    unsafe fn is_stick_forward(&mut self) -> bool{
        let stick_value_x = ControlModule::get_stick_x(self);
        if stick_value_x != 0. {
            if stick_value_x*PostureModule::lr(self) > 0. {
                return true;
            }
        }
        return false;
    }

    unsafe fn get_entry_id(&mut self) -> usize {
        WorkModule::get_int(self, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize
    }
    unsafe fn status_kind(&mut self) -> i32 {
        StatusModule::status_kind(self)
    }

    /// returns whether or not the stick x is pointed in the "backwards" direction for
    /// a character
    unsafe fn is_stick_backward(&mut self) -> bool{
        let stick_value_x = ControlModule::get_stick_x(self);
        if stick_value_x != 0. {
            if stick_value_x*PostureModule::lr(self) < 0. {
                return true;
            }
        }
        return false;
    }

    unsafe fn is_status(&mut self, kind: i32) -> bool {
        return StatusModule::status_kind(self) == kind;
    }

    unsafe fn is_status_one_of(&mut self, kinds: &[i32]) -> bool {
        let kind = StatusModule::status_kind(self);
        return kinds.contains(&kind);
    }

    unsafe fn is_prev_status(&mut self, kind: i32) -> bool {
        return StatusModule::prev_status_kind(self, 0) == kind;
    }

    unsafe fn set_gravity(&mut self, disable: bool) {
        if disable{
            KineticModule::unable_energy(self, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        else{
            KineticModule::enable_energy(self, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }

    unsafe fn is_prev_status_one_of(&mut self, kinds: &[i32]) -> bool {
        let kind = StatusModule::prev_status_kind(self, 0);
        return kinds.contains(&kind);
    }

    unsafe fn is_situation(&mut self, kind: i32) -> bool {
        return StatusModule::situation_kind(self) == kind;
    }

    unsafe fn is_prev_situation(&mut self, kind: i32) -> bool {
        return StatusModule::prev_situation_kind(self) == kind;
    }

    unsafe fn is_motion(&mut self, kind: Hash40) -> bool {
        return MotionModule::motion_kind(self) == kind.hash;
    }

    unsafe fn is_motion_one_of(&mut self, kinds: &[Hash40]) -> bool {
        let kind = MotionModule::motion_kind(self);
        return kinds.contains(&Hash40::new_raw(kind));
    }

    /// gets the current status kind for the fighter
    unsafe fn status(&mut self) -> i32 {
        return StatusModule::status_kind(self);
    }

    unsafe fn get_num_used_jumps(&mut self) -> i32 {
        return WorkModule::get_int(self, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }

    unsafe fn get_jump_count_max(&mut self) -> i32 {
        return WorkModule::get_int(self, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
    }

    unsafe fn motion_frame(&mut self) -> f32 {
        return MotionModule::frame(self);
    }

    unsafe fn set_rate(&mut self, motion_rate: f32) {
        MotionModule::set_rate(self, motion_rate);
    }

    unsafe fn set_scale(&mut self, scale: f32) {
        ModelModule::set_scale(self, scale);
    }

    unsafe fn set_joint_scale(&mut self, joint: Hash40, scale: *const Vector3f) {
        ModelModule::set_joint_scale(self, joint, scale);
    }

    unsafe fn is_in_hitlag(&mut self) -> bool{
        let hitlag_frame = WorkModule::get_int(self, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME);
        if hitlag_frame > 0 {
            return true;
        }
        return false;
    }

    unsafe fn get_owner_boma(&mut self) -> *mut BattleObjectModuleAccessor {
        smash::app::sv_battle_object::module_accessor(WorkModule::get_int(self, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32)
    }

    unsafe fn change_status(&mut self, kind: i32, repeat: bool) -> i32 {
        return StatusModule::change_status_request_from_script(self, kind, repeat) as i32;
    }

    unsafe fn is_fighter(&mut self) -> bool {
        return smash::app::utility::get_category(self) == *BATTLE_OBJECT_CATEGORY_FIGHTER;
    }

    unsafe fn is_weapon(&mut self) -> bool {
        return smash::app::utility::get_category(self) == *BATTLE_OBJECT_CATEGORY_WEAPON;
    }

    unsafe fn kind(&mut self) -> i32 {
        return smash::app::utility::get_kind(self);
    }

    unsafe fn get_int(&mut self, what: i32) -> i32 {
        WorkModule::get_int(self, what)
    }

    unsafe fn get_float(&mut self, what: i32) -> f32 {
        WorkModule::get_float(self, what)
    }

    unsafe fn get_int64(&mut self, what: i32) -> u64 {
        WorkModule::get_int64(self, what)
    }

    unsafe fn is_flag(&mut self, what: i32) -> bool {
        WorkModule::is_flag(self, what)
    }

    unsafe fn set_int(&mut self, value: i32, what: i32) {
        WorkModule::set_int(self, value, what)
    }

    unsafe fn set_float(&mut self, value: f32, what: i32) {
        WorkModule::set_float(self, value, what)
    }


    unsafe fn set_int64(&mut self, value: i64, what: i32) {
        WorkModule::set_int64(self, value, what)
    }


    unsafe fn on_flag(&mut self, what: i32) {
        WorkModule::on_flag(self, what)
    }

    unsafe fn off_flag(&mut self, what: i32) {
        WorkModule::off_flag(self, what)
    }


    unsafe fn get_param_int(&mut self, obj: &str, field: &str) -> i32 {
        WorkModule::get_param_int(self, Hash40::new(obj).hash, Hash40::new(field).hash)
    }

    unsafe fn get_param_float(&mut self, obj: &str, field: &str) -> f32 {
        WorkModule::get_param_float(self, Hash40::new(obj).hash, Hash40::new(field).hash)
    }

    unsafe fn get_param_int64(&mut self, obj: &str, field: &str) -> u64 {
        WorkModule::get_param_int64(self, Hash40::new(obj).hash, Hash40::new(field).hash)
    }

    /// gets the FighterKineticEnergyMotion object
    unsafe fn get_motion_energy(&mut self) -> &mut FighterKineticEnergyMotion {
        std::mem::transmute::<u64, &mut app::FighterKineticEnergyMotion>(KineticModule::get_energy(self, *FIGHTER_KINETIC_ENERGY_ID_MOTION))
    }

    /// gets the FighterKineticEnergyController object
    unsafe fn get_controller_energy(&mut self) -> &mut FighterKineticEnergyController {
        std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(self, *FIGHTER_KINETIC_ENERGY_ID_CONTROL))
    }

    unsafe fn set_color_rgb(&mut self, r: f32, g: f32, b: f32, model_color_type: ModelColorType){
        set_color_rgb(self, r, g, b, model_color_type)
    }
}