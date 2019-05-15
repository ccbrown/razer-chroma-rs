extern crate arraymap;

use arraymap::ArrayMap;

mod bindings_raw {
    #![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, dead_code)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use bindings_raw::root as bindings;

pub struct SDK;

pub struct Effect {
    id: bindings::RZEFFECTID,
}

impl Effect {
    fn new(id: bindings::RZEFFECTID) -> Effect {
        Effect{
            id: id,
        }
    }
}

impl Drop for Effect {
	fn drop(&mut self) {
		unsafe {
			bindings::ChromaSDKDeleteEffect(self.id);
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Color(bindings::RZCOLOR);

impl Color {
	pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
		Color((r as bindings::RZCOLOR) | ((g as bindings::RZCOLOR) << 8) | ((b as bindings::RZCOLOR) << 16))
	}
}

pub struct Error {
	pub code: bindings::RZRESULT,
}

pub enum ChromaLinkEffect {
	Static{color: Color},
	Custom{color: [Color; bindings::ChromaSDK::ChromaLink::MAX_LEDS]},
}

pub enum HeadsetEffect {
	Static{color: Color},
	Custom{color: [Color; bindings::ChromaSDK::Headset::MAX_LEDS]},
}

pub enum KeyboardEffect {
	Static{color: Color},
	Custom{color: [[Color; bindings::ChromaSDK::Keyboard::MAX_COLUMN]; bindings::ChromaSDK::Keyboard::MAX_ROW]},
}

pub enum KeypadEffect {
	Static{color: Color},
	Custom{color: [[Color; bindings::ChromaSDK::Keypad::MAX_COLUMN]; bindings::ChromaSDK::Keypad::MAX_ROW]},
}

pub enum MouseEffect {
	Static{color: Color},
	Custom{color: [[Color; bindings::ChromaSDK::Mouse::MAX_COLUMN]; bindings::ChromaSDK::Mouse::MAX_ROW]},
}

pub enum MousepadEffect {
	Static{color: Color},
	Custom{color: [Color; bindings::ChromaSDK::Mousepad::MAX_LEDS]},
}

impl SDK {
    pub fn new() -> Option<SDK> {
		unsafe {
	        if bindings::Load() && bindings::ChromaSDKInit() == bindings::ERROR_SUCCESS as _ {
				Some(SDK{})
			} else {
				None
			}
		}
    }

	pub fn create_chroma_link_effect(&mut self, effect: ChromaLinkEffect) -> Result<Effect, Error> {
		let mut effect_id = bindings::RZEFFECTID{Data1: 0, Data2: 0, Data3: 0, Data4: [0, 0, 0, 0, 0, 0, 0, 0]};
		let code = unsafe {
			match effect {
				ChromaLinkEffect::Static{color} => {
					let mut param = bindings::ChromaSDK::ChromaLink::STATIC_EFFECT_TYPE{Color: color.0};
					bindings::ChromaSDKCreateChromaLinkEffect(bindings::ChromaSDK::ChromaLink::EFFECT_TYPE_CHROMA_STATIC, &mut param as *mut _ as *mut std::ffi::c_void, &mut effect_id)
				}
				ChromaLinkEffect::Custom{color} => {
					let mut param = bindings::ChromaSDK::ChromaLink::CUSTOM_EFFECT_TYPE{Color: color.map(|v| v.0)};
					bindings::ChromaSDKCreateChromaLinkEffect(bindings::ChromaSDK::ChromaLink::EFFECT_TYPE_CHROMA_CUSTOM, &mut param as *mut _ as *mut std::ffi::c_void, &mut effect_id)
				}
			}
		};
		if code == bindings::ERROR_SUCCESS as _ {
			Ok(Effect::new(effect_id))
		} else {
			Err(Error{code: code})
		}
	}

	pub fn create_headset_effect(&mut self, effect: HeadsetEffect) -> Result<Effect, Error> {
		let mut effect_id = bindings::RZEFFECTID{Data1: 0, Data2: 0, Data3: 0, Data4: [0, 0, 0, 0, 0, 0, 0, 0]};
		let code = unsafe {
			match effect {
				HeadsetEffect::Static{color} => {
					let mut param = bindings::ChromaSDK::Headset::STATIC_EFFECT_TYPE{Color: color.0};
					bindings::ChromaSDKCreateHeadsetEffect(bindings::ChromaSDK::Headset::EFFECT_TYPE_CHROMA_STATIC, &mut param as *mut _ as *mut std::ffi::c_void, &mut effect_id)
				}
				HeadsetEffect::Custom{color} => {
					let mut param = bindings::ChromaSDK::Headset::CUSTOM_EFFECT_TYPE{Color: color.map(|v| v.0)};
					bindings::ChromaSDKCreateHeadsetEffect(bindings::ChromaSDK::Headset::EFFECT_TYPE_CHROMA_CUSTOM, &mut param as *mut _ as *mut std::ffi::c_void, &mut effect_id)
				}
			}
		};
		if code == bindings::ERROR_SUCCESS as _ {
			Ok(Effect::new(effect_id))
		} else {
			Err(Error{code: code})
		}
	}

	pub fn create_keyboard_effect(&mut self, effect: KeyboardEffect) -> Result<Effect, Error> {
		let mut effect_id = bindings::RZEFFECTID{Data1: 0, Data2: 0, Data3: 0, Data4: [0, 0, 0, 0, 0, 0, 0, 0]};
		let code = unsafe {
			match effect {
				KeyboardEffect::Static{color} => {
					let mut param = bindings::ChromaSDK::Keyboard::STATIC_EFFECT_TYPE{Color: color.0};
					bindings::ChromaSDKCreateKeyboardEffect(bindings::ChromaSDK::Keyboard::EFFECT_TYPE_CHROMA_STATIC, &mut param as *mut _ as *mut std::ffi::c_void, &mut effect_id)
				}
				KeyboardEffect::Custom{color} => {
					let mut param = bindings::ChromaSDK::Keyboard::CUSTOM_EFFECT_TYPE{Color: color.map(|v| v.map(|v| v.0))};
					bindings::ChromaSDKCreateKeyboardEffect(bindings::ChromaSDK::Keyboard::EFFECT_TYPE_CHROMA_CUSTOM, &mut param as *mut _ as *mut std::ffi::c_void, &mut effect_id)
				}
			}
		};
		if code == bindings::ERROR_SUCCESS as _ {
			Ok(Effect::new(effect_id))
		} else {
			Err(Error{code: code})
		}
	}

	pub fn create_keypad_effect(&mut self, effect: KeypadEffect) -> Result<Effect, Error> {
		let mut effect_id = bindings::RZEFFECTID{Data1: 0, Data2: 0, Data3: 0, Data4: [0, 0, 0, 0, 0, 0, 0, 0]};
		let code = unsafe {
			match effect {
				KeypadEffect::Static{color} => {
					let mut param = bindings::ChromaSDK::Keypad::STATIC_EFFECT_TYPE{Color: color.0};
					bindings::ChromaSDKCreateKeypadEffect(bindings::ChromaSDK::Keypad::EFFECT_TYPE_CHROMA_STATIC, &mut param as *mut _ as *mut std::ffi::c_void, &mut effect_id)
				}
				KeypadEffect::Custom{color} => {
					let mut param = bindings::ChromaSDK::Keypad::CUSTOM_EFFECT_TYPE{Color: color.map(|v| v.map(|v| v.0))};
					bindings::ChromaSDKCreateKeypadEffect(bindings::ChromaSDK::Keypad::EFFECT_TYPE_CHROMA_CUSTOM, &mut param as *mut _ as *mut std::ffi::c_void, &mut effect_id)
				}
			}
		};
		if code == bindings::ERROR_SUCCESS as _ {
			Ok(Effect::new(effect_id))
		} else {
			Err(Error{code: code})
		}
	}

	pub fn create_mouse_effect(&mut self, effect: MouseEffect) -> Result<Effect, Error> {
		let mut effect_id = bindings::RZEFFECTID{Data1: 0, Data2: 0, Data3: 0, Data4: [0, 0, 0, 0, 0, 0, 0, 0]};
		let code = unsafe {
			match effect {
				MouseEffect::Static{color} => {
					let mut param = bindings::ChromaSDK::Mouse::STATIC_EFFECT_TYPE{LEDId: bindings::ChromaSDK::Mouse::RZLED_RZLED_ALL, Color: color.0};
					bindings::ChromaSDKCreateMouseEffect(bindings::ChromaSDK::Mouse::EFFECT_TYPE_CHROMA_STATIC, &mut param as *mut _ as *mut std::ffi::c_void, &mut effect_id)
				}
				MouseEffect::Custom{color} => {
					let mut param = bindings::ChromaSDK::Mouse::CUSTOM_EFFECT_TYPE2{Color: color.map(|v| v.map(|v| v.0))};
					bindings::ChromaSDKCreateMouseEffect(bindings::ChromaSDK::Mouse::EFFECT_TYPE_CHROMA_CUSTOM, &mut param as *mut _ as *mut std::ffi::c_void, &mut effect_id)
				}
			}
		};
		if code == bindings::ERROR_SUCCESS as _ {
			Ok(Effect::new(effect_id))
		} else {
			Err(Error{code: code})
		}
	}

	pub fn create_mousepad_effect(&mut self, effect: MousepadEffect) -> Result<Effect, Error> {
		let mut effect_id = bindings::RZEFFECTID{Data1: 0, Data2: 0, Data3: 0, Data4: [0, 0, 0, 0, 0, 0, 0, 0]};
		let code = unsafe {
			match effect {
				MousepadEffect::Static{color} => {
					let mut param = bindings::ChromaSDK::Mousepad::STATIC_EFFECT_TYPE{Color: color.0};
					bindings::ChromaSDKCreateMousepadEffect(bindings::ChromaSDK::Mousepad::EFFECT_TYPE_CHROMA_STATIC, &mut param as *mut _ as *mut std::ffi::c_void, &mut effect_id)
				}
				MousepadEffect::Custom{color} => {
					let mut param = bindings::ChromaSDK::Mousepad::CUSTOM_EFFECT_TYPE{Color: color.map(|v| v.0)};
					bindings::ChromaSDKCreateMousepadEffect(bindings::ChromaSDK::Mousepad::EFFECT_TYPE_CHROMA_CUSTOM, &mut param as *mut _ as *mut std::ffi::c_void, &mut effect_id)
				}
			}
		};
		if code == bindings::ERROR_SUCCESS as _ {
			Ok(Effect::new(effect_id))
		} else {
			Err(Error{code: code})
		}
	}

	pub fn set_effect(&mut self, effect: &Effect) -> Result<(), Error> {
		let code = unsafe {
			bindings::ChromaSDKSetEffect(effect.id)
		};
		if code == bindings::ERROR_SUCCESS as _ {
			Ok(())
		} else {
			Err(Error{code: code})
		}
	}
}

impl Drop for SDK {
	fn drop(&mut self) {
		unsafe {
			bindings::ChromaSDKUnInit();
		}
	}
}
