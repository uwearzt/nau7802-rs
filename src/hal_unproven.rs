use crate::Nau7802;

use embedded_hal::{
    adc::{Channel, OneShot},
    blocking::i2c,
};

pub struct Channel0;

impl<D: i2c::Read + i2c::Write> OneShot<Self, i32, Channel0> for Nau7802<D> {
    type Error = crate::Error;

    fn read(&mut self, _: &mut Channel0) -> nb::Result<i32, Self::Error> {
        self.read()
    }
}

impl<D: i2c::Read + i2c::Write> Channel<Nau7802<D>> for Channel0 {
    type ID = ();
    fn channel() -> Self::ID {
        ()
    }
}
