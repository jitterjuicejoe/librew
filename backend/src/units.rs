use std::marker::PhantomData;
use std::ops::Add;

pub trait Unit {}
pub trait Quantity {}

pub struct Scalar<T, Q: Quantity, U: Unit> {
    value: T,
    q: PhantomData<Q>,
    u: PhantomData<U>,
}

impl<T: Add<Output = T>, Q: Quantity, U: Unit> Add for Scalar<T, Q, U> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
            q: PhantomData,
            u: PhantomData,
        }
    }
}

impl<T, Q: Quantity, U: Unit> From<T> for Scalar<T, Q, U> {
    fn from(value: T) -> Self {
        Self {
            value,
            q: PhantomData,
            u: PhantomData,
        }
    }
}

impl<T: Copy, Q: Quantity, U: Unit> Scalar<T, Q, U> {
    pub fn raw(&self) -> T {
        self.value
    }
}

pub struct Celcius;
pub struct Kelvin;
pub struct Temperature;

impl Unit for Celcius {}
impl Unit for Kelvin {}
impl Quantity for Temperature {}

pub type TemperatureKelvin = Scalar<f64, Temperature, Kelvin>;
pub type TemperatureCelcius = Scalar<f64, Temperature, Celcius>;
