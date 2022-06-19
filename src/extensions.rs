//most of this is from https://github.com/HDR-Development/HewDraw-Remix

use smash::app::{
    self,
    *,
    lua_bind::*,
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

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AerialKind {
    Nair,
    Fair,
    Bair,
    Uair,
    Dair
}

pub trait BomaExt {
    // INPUTS
    unsafe fn clear_commands<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T);
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

    /// returns whether or not the stick x is pointed in the "forwards" direction for
    /// a character
    unsafe fn is_stick_forward(&mut self) -> bool;

    /// returns whether or not the stick x is pointed in the "backwards" direction for
    /// a character
    unsafe fn is_stick_backward(&mut self) -> bool;

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

    /// gets the number of jumps that have been used
    unsafe fn get_num_used_jumps(&mut self) -> i32;

    /// gets the max allowed number of jumps for this character
    unsafe fn get_jump_count_max(&mut self) -> i32;
    unsafe fn motion_frame(&mut self) -> f32;
    unsafe fn set_rate(&mut self, motion_rate: f32);
    unsafe fn set_scale(&mut self, scale: f32);
    unsafe fn set_joint_scale(&mut self, joint: Hash40, scale: *const Vector3f);
    unsafe fn is_in_hitlag(&mut self) -> bool;

    unsafe fn get_owner_boma(&mut self) -> BattleObjectModuleAccessor;

    unsafe fn change_status_req(&mut self, kind: i32, repeat: bool) -> i32;

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

impl BomaExt for BattleObjectModuleAccessor {
    unsafe fn clear_commands<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) {
        let cat = fighter_pad_cmd_flag.into();
        let (cat, bits) = match cat {
            CommandCat::Cat1(cat) => (0, cat.bits()),
            CommandCat::Cat2(cat) => (1, cat.bits()),
            CommandCat::Cat3(cat) => (2, cat.bits()),
            CommandCat::Cat4(cat) => (3, cat.bits()),
            CommandCat::CatHdr(cat) => (4, cat.bits())
        };

        crate::modules::InputModule::clear_commands(self.object(), cat, bits);
    }

    unsafe fn is_cat_flag<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool {
        let cat = fighter_pad_cmd_flag.into();
        match cat {
            CommandCat::Cat1(cat) => Cat1::new(self).intersects(cat),
            CommandCat::Cat2(cat) => Cat2::new(self).intersects(cat),
            CommandCat::Cat3(cat) => Cat3::new(self).intersects(cat),
            CommandCat::Cat4(cat) => Cat4::new(self).intersects(cat),
            CommandCat::CatHdr(cat) => CatHdr::new(self).intersects(cat)
        }
    }

    unsafe fn is_cat_flag_all<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool {
        let cat = fighter_pad_cmd_flag.into();
        match cat {
            CommandCat::Cat1(cat) => Cat1::new(self).contains(cat),
            CommandCat::Cat2(cat) => Cat2::new(self).contains(cat),
            CommandCat::Cat3(cat) => Cat3::new(self).contains(cat),
            CommandCat::Cat4(cat) => Cat4::new(self).contains(cat),
            CommandCat::CatHdr(cat) => CatHdr::new(self).intersects(cat)
        }
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

    unsafe fn is_input_jump(&mut self) -> bool {
        if self.is_cat_flag(Cat1::Jump) && ControlModule::is_enable_flick_jump(self) {
            WorkModule::set_int(self, 1, *FIGHTER_INSTANCE_WORK_ID_INT_STICK_JUMP_COMMAND_LIFE);
            return true;
        }

        return self.is_cat_flag(Cat1::JumpButton);
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

    unsafe fn set_rate(&mut self, motion_rate: f32) {
        MotionModule::set_rate(self, motion_rate);
    }

    unsafe fn is_motion_one_of(&mut self, kinds: &[Hash40]) -> bool {
        let kind = MotionModule::motion_kind(self);
        return kinds.contains(&Hash40::new_raw(kind));
    }

    unsafe fn motion_frame(&mut self) -> f32 {
        return MotionModule::frame(self);
    }

    unsafe fn is_in_hitlag(&mut self) -> bool{
        let hitlag_frame = WorkModule::get_int(self, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME);
        if hitlag_frame > 0 {
            return true;
        }
        return false;
    }

    unsafe fn change_status_req(&mut self, kind: i32, repeat: bool) -> i32 {
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

    unsafe fn get_num_used_jumps(&mut self) -> i32 {
        return WorkModule::get_int(self, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }

    unsafe fn get_jump_count_max(&mut self) -> i32 {
        return WorkModule::get_int(self, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
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


    unsafe fn set_joint_rotate(&mut self, bone_name: &str, rotation: Vector3f) {
        ModelModule::set_joint_rotate(self, Hash40::new(&bone_name), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8})
    }


    /// gets the FighterKineticEnergyMotion object
    unsafe fn get_motion_energy(&mut self) -> &mut FighterKineticEnergyMotion {
        std::mem::transmute::<u64, &mut app::FighterKineticEnergyMotion>(KineticModule::get_energy(self, *FIGHTER_KINETIC_ENERGY_ID_MOTION))
    }

    /// gets the FighterKineticEnergyController object
    unsafe fn get_controller_energy(&mut self) -> &mut FighterKineticEnergyController {
        std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(self, *FIGHTER_KINETIC_ENERGY_ID_CONTROL))
    }


    /// gets the current status kind for the fighter
    unsafe fn status(&mut self) -> i32 {
        return StatusModule::status_kind(self);
    }

    unsafe fn get_owner_boma(&mut self) -> BattleObjectModuleAccessor {
        *smash::app::sv_battle_object::module_accessor((WorkModule::get_int(self, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32)
    }

    unsafe fn set_scale(&mut self, scale: f32) {
        ModelModule::set_scale(self, scale);
    }

    unsafe fn set_joint_scale(&mut self, joint: Hash40, scale: *const Vector3f) {
        ModelModule::set_joint_scale(self, joint, scale);
    }
}