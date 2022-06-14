use voladdress::*;

pub const DISPCNT: VolAddress<DisplayControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0000) };

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DisplayControl(u16);

impl DisplayControl {
  pub_const_fn_new!();
  unsafe_u16_enum_field!(0 - 2: DisplayMode, display_mode, with_display_mode);
  u16_bool_field!(4, frame_select, with_frame_select);
  u16_bool_field!(5, hblank_oam_free, with_hblank_oam_free);
  u16_bool_field!(6, obj_vram_1d, with_obj_vram_1d);
  u16_bool_field!(7, forced_blank, with_forced_blank);
  u16_bool_field!(8, display_bg0, with_display_bg0);
  u16_bool_field!(9, display_bg1, with_display_bg1);
  u16_bool_field!(10, display_bg2, with_display_bg2);
  u16_bool_field!(11, display_bg3, with_display_bg3);
  u16_bool_field!(12, display_obj, with_display_obj);
  u16_bool_field!(13, display_win0, with_display_win0);
  u16_bool_field!(14, display_win1, with_display_win1);
  u16_bool_field!(15, display_obj_win, with_display_obj_win);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum DisplayMode {
  #[default]
  _0 = 0,
  _1 = 1,
  _2 = 2,
  _3 = 3,
  _4 = 4,
  _5 = 5,
}
