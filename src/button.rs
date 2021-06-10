#![allow(dead_code)]

use core::convert::Infallible;

use embedded_hal::digital::v2::InputPin;

use crate::{prelude::*, switch::Switch};

const HIGH_PULL: u8 = 0;
const LOW_PULL: u8 = 1;
const NORM_OPEN: u8 = 0;
const NORM_CLOSE: u8 = 1;
const MANUAL: u8 = 0;
const AUTO: u8 = 1;

pub struct ButtonFlags {
    btn_deb: bool,
    hold: bool,
    counter: bool,
    is_holded: bool,
    is_release: bool,
    is_press: bool,
    step_flag: bool,
    one_click: bool,
    is_one: bool,
    mode: bool,
    tick_mode: bool,
    no_pin: bool,
    counter_reset: bool,
}

pub struct Button<P, D>
where
    D: Direction,
{
    pin: Switch<P, D>,
    flags: ButtonFlags,
    btn_counter: u8,
    last_counter: u32,
    last_hold_counter: u32,
    btn_timer: u128,
    btn_state: bool,
    btn_flag: bool,
    debounce: u16,
    timeout: u16,
    click_timeout: u16,
    step_timeout: u16,
}

impl<P, D> Button<P, D>
where
    D: Direction,
{
    pub fn new(pin: P) -> Self {
        Self {
            pin: pin.into_switch(),
            flags: ButtonFlags {
                btn_deb: false,
                hold: false,
                counter: false,
                is_holded: false,
                is_release: false,
                is_press: false,
                step_flag: false,
                one_click: false,
                is_one: false,
                mode: false,
                tick_mode: false,
                no_pin: false,
                counter_reset: false,
            },
            btn_counter: 0,
            last_counter: 0,
            last_hold_counter: 0,
            btn_timer: 0,
            btn_state: false,
            btn_flag: false,
            debounce: 60,
            timeout: 500,
            click_timeout: 500,
            step_timeout: 400,
        }
    }
}

impl<P, D> Button<P, D>
where
    P: InputPin,
    D: Direction,
{
    pub fn set_debounce(&mut self, debounce: u16) {
        self.debounce = debounce;
    }

    pub fn set_timeout(&mut self, new_timeout: u16) {
        self.timeout = new_timeout;
    }

    pub fn set_click_timeout(&mut self, new_timeout: u16) {
        self.click_timeout = new_timeout;
    }

    pub fn set_step_timeout(&mut self, step_timeout: u16) {
        self.step_timeout = step_timeout;
    }

    pub fn set_tick_mode(&mut self, tick_mode: bool) {
        self.flags.tick_mode = tick_mode;
    }

    pub fn is_press(&mut self) -> bool {
        /*if self.flags.tick_mode {
            self.tick();
        }*/
        if self.flags.is_press {
            self.flags.is_press = false;
            true
        } else {
            false
        }
    }

    pub fn is_release(&mut self) -> bool {
        /*if self.flags.tick_mode {
            self.tick();
        }*/
        if self.flags.is_release {
            self.flags.is_release = false;
            true
        } else {
            false
        }
    }

    pub fn is_click(&mut self) -> bool {
        /*if self.flags.tick_mode {
            self.tick();
        }*/
        if self.flags.is_one {
            self.flags.is_one = false;
            true
        } else {
            false
        }
    }

    pub fn is_holded(&mut self) -> bool {
        /*if self.flags.tick_mode {
            self.tick();
        }*/
        if self.flags.is_holded {
            self.flags.is_holded = false;
            true
        } else {
            false
        }
    }

    pub fn is_hold(&mut self) -> bool {
        /*if self.flags.tick_mode {
            self.tick();
        }*/
        self.flags.step_flag
    }

    pub fn state(&mut self) -> bool {
        /*if self.flags.tick_mode {
            self.tick();
        }*/
        self.btn_state
    }

    pub fn is_single(&mut self) -> bool {
        /*if self.flags.tick_mode {
            self.tick();
        }*/
        if self.flags.counter && self.last_counter == 1 {
            //		last_counter = 0;
            //		flags.counter_flag = false;
            self.flags.counter_reset = true;
            true
        } else {
            false
        }
    }

    pub fn is_double(&mut self) -> bool {
        /*if self.flags.tick_mode {
            self.tick();
        }*/
        if self.flags.counter && self.last_counter == 2 {
            //		last_counter = 0;
            //		flags.counter_flag = false;
            self.flags.counter_reset = true;
            true
        } else {
            false
        }
    }

    pub fn is_triple(&mut self) -> bool {
        /*if self.flags.tick_mode {
            self.tick();
        }*/
        if self.flags.counter && self.last_counter == 3 {
            //		flags.counter_flag = false;
            //		last_counter = 0;
            self.flags.counter_reset = true;
            true
        } else {
            false
        }
    }

    pub fn has_clicks(&mut self) -> bool {
        /*if self.flags.tick_mode {
            self.tick();
        }*/
        if self.flags.counter {
            self.flags.counter_reset = true;
            true
        } else {
            false
        }
    }

    pub fn get_clicks(&mut self) -> u32 {
        self.flags.counter_reset = true;
        self.last_counter
    }

    pub fn get_hold_clicks(&mut self) -> u32 {
        /*if self.flags.tick_mode {
            self.tick();
        }*/
        self.last_hold_counter
    }

    /*pub fn is_step(&mut self, clicks: u8) -> bool {
        /*if self.flags.tick_mode {
            self.tick();
        }*/
        if self.btn_counter == clicks
            && self.flags.step_flag
            && (self.uptime.get().as_millis() - self.btn_timer >= self.step_timeout as u128)
        {
            self.btn_timer = self.uptime.get().as_millis();
            true
        } else {
            false
        }
    }*/

    pub fn reset_states(&mut self) {
        self.flags.is_press = false;
        self.flags.is_release = false;
        self.flags.is_one = false;
        self.flags.is_holded = false;
        self.flags.step_flag = false;
        self.flags.counter = false;
        self.last_hold_counter = 0;
        self.last_counter = 0;
    }

    /*pub fn tick_with_state(&mut self, state: bool) {
        self.flags.mode = true;
        self.btn_state = state ^ self.flags.inv_state;
        self.tick();
        self.flags.mode = false;
    }*/
}

impl<P, D, U> TryTickWithResource<&U> for Button<P, D>
where
    P: InputPin,
    D: Direction,
    U: Uptime,
{
    type Error = <P as InputPin>::Error;

    fn try_tick_with_resource(&mut self, uptime: &U) -> Result<(), Self::Error> {
        // читаем пин
        self.btn_state = self.pin.try_check_is_disabled()?;

        let this_mls = uptime.get().as_millis();

        // нажатие
        if self.btn_state && !self.btn_flag {
            if !self.flags.btn_deb {
                self.flags.btn_deb = true;
                self.btn_timer = this_mls;
            } else {
                if this_mls - self.btn_timer >= self.debounce as u128 {
                    self.btn_flag = true;
                    self.flags.is_press = true;
                    self.flags.one_click = true;
                }
            }
        } else {
            self.flags.btn_deb = false;
        }

        // отпускание
        if !self.btn_state && self.btn_flag {
            self.btn_flag = false;
            if !self.flags.hold {
                self.btn_counter += 1;
            }
            self.flags.hold = false;
            self.flags.is_release = true;
            self.btn_timer = this_mls;
            if self.flags.step_flag {
                self.last_counter = 0;
                self.btn_counter = 0;
                self.flags.step_flag = false;
            }
            if self.flags.one_click {
                self.flags.one_click = false;
                self.flags.is_one = true;
            }
        }

        // кнопка удерживается
        if self.btn_flag
            && self.btn_state
            && (this_mls - self.btn_timer >= self.timeout as u128)
            && !self.flags.hold
        {
            self.flags.hold = true;
            self.last_hold_counter = self.btn_counter as u32;
            //self.btn_counter = 0;
            //self.last_counter = 0;
            self.flags.is_holded = true;
            self.flags.step_flag = true;
            self.flags.one_click = false;
            self.btn_timer = this_mls;
        }

        // обработка накликивания
        if (this_mls - self.btn_timer >= self.click_timeout as u128)
            && (self.btn_counter != 0)
            && !self.btn_state
        {
            //И здесь еще добавлен !self.btn_state
            self.last_counter = self.btn_counter as u32;
            self.btn_counter = 0;
            self.flags.counter = true;
        }

        // сброс накликивания						//Добавлено
        if self.flags.counter_reset {
            self.last_counter = 0;
            self.flags.counter = false;
            self.flags.counter_reset = false;
        }

        Ok(())
    }
}

impl<P, D, U> TickWithResource<&U> for Button<P, D>
where
    P: InputPin<Error = Infallible>,
    D: Direction,
    U: Uptime,
{
    fn tick_with_resource(&mut self, uptime: &U) {
        if let Err(e) = self.try_tick_with_resource(uptime) {
            match e {}
        };
    }
}
