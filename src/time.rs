use core::time::Duration;

pub trait Uptime {
    fn get(&self) -> Duration;
}

#[cfg(feature = "cortex_m")]
#[macro_export]
macro_rules! cortex_m_uptime {
    ($timer:ty => $visibility:vis $name:ident as $duration_from:expr) => {
        $visibility use _uptime_internals::$name;

        mod _uptime_internals {
            use super::*;

            use core::cell::{Cell, RefCell};

            use $crate::time::Uptime;
            use cortex_m::interrupt::Mutex;
            use embedded_hal::timer::CountDown;
            use once_cell::unsync::OnceCell;

            static COUNTER: Mutex<Cell<u64>> = Mutex::new(Cell::new(0));
            static TIMER: Mutex<OnceCell<RefCell<$timer>>> = Mutex::new(OnceCell::new());

            #[derive(Copy, Clone, Debug, Default)]
            pub struct $name {
                _priv: (),
            }

            impl $name {
                pub unsafe fn timer_interruption_callback() {
                    cortex_m::interrupt::free(|cs| {
                        if let Some(timer) = TIMER.borrow(cs).get() {
                            let mut borrowed_timer = timer.borrow_mut();
                            let borrowed_counter = COUNTER.borrow(cs);

                            borrowed_counter.set(borrowed_counter.get() + 1);
                            borrowed_timer.wait().unwrap();
                        }
                    })
                }

                pub unsafe fn initialize_with_timer(timer: $timer) -> Option<Self> {
                    cortex_m::interrupt::free(|cs| {
                        let borrowed_timer = TIMER.borrow(cs);
                        borrowed_timer
                            .set(RefCell::new(timer))
                            .ok()
                            .map(|_| Self { _priv: () })
                    })
                }
            }

            impl Uptime for $name {
                fn get(&self) -> Duration {
                    cortex_m::interrupt::free(|cs| {
                        $duration_from(COUNTER.borrow(cs).get())
                    })
                }
            }
        }
    };
}
