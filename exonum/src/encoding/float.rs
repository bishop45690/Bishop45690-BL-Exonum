// Copyright 2017 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::mem;
use std::error::Error;

use byteorder::{ByteOrder, LittleEndian};
use serde_json::value::{Value, Number};

use super::Result as EncodingResult;
use super::Error as EncodingError;
use encoding::{CheckedOffset, Field, Offset};
use encoding::serialize::WriteBufferWrapper;
use encoding::serialize::json::ExonumJson;

/// Wrapper for the `f32` type that restricts non-finite (NaN and Infinity) values.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct F32 {
    value: f32,
}

impl F32 {
    /// Creates a new `F32` instance with the given `value`.
    ///
    /// # Panics
    ///
    /// Panics if `is_finite()` returns `false` for the given `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// use exonum::encoding::F32;
    ///
    /// let val = F32::new(1.0);
    /// assert_eq!(val.get(), 1.0);
    /// ```
    pub fn new(value: f32) -> Self {
        Self::try_from(value).expect("Unexpected non-finite value")
    }

    /// Creates a new `F32` instance with the given `value`. Returns `None` if the given value
    /// isn't finite.
    ///
    /// # Examples
    ///
    /// ```
    /// use exonum::encoding::F32;
    /// use std::f32;
    ///
    /// let val = F32::try_from(1.0);
    /// assert!(val.is_some());
    ///
    /// let val = F32::try_from(f32::NAN);
    /// assert!(val.is_none());
    /// ```
    pub fn try_from(value: f32) -> Option<Self> {
        if value.is_finite() {
            Some(Self { value })
        } else {
            None
        }
    }

    /// Returns value contained in this wrapper.
    ///
    /// # Examples
    ///
    /// ```
    /// use exonum::encoding::F32;
    ///
    /// let wrapper = F32::new(1.0);
    /// let value = wrapper.get();
    /// ```
    pub fn get(&self) -> f32 {
        self.value
    }
}

/// Wrapper for the `f64` type that restricts non-numeric (NaN and Infinity) values.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct F64 {
    value: f64,
}

impl F64 {
    /// Creates a new `F64` instance with the given `value`.
    ///
    /// # Panics
    ///
    /// Panics if `is_finite()` returns `false` for the given `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// use exonum::encoding::F64;
    ///
    /// let val = F64::new(1.0);
    /// assert_eq!(val.get(), 1.0);
    /// ```
    pub fn new(value: f64) -> Self {
        Self::try_from(value).expect("Unexpected non-finite value")
    }

    /// Creates a new `F64` instance with the given `value`. Returns `None` if the given value
    /// isn't finite.
    ///
    /// # Examples
    ///
    /// ```
    /// use exonum::encoding::F64;
    /// use std::f64;
    ///
    /// let val = F64::try_from(1.0);
    /// assert!(val.is_some());
    ///
    /// let val = F64::try_from(f64::NAN);
    /// assert!(val.is_none());
    /// ```
    pub fn try_from(value: f64) -> Option<Self> {
        if value.is_finite() {
            Some(Self { value })
        } else {
            None
        }
    }

    /// Returns value contained in this wrapper.
    ///
    /// # Examples
    ///
    /// ```
    /// use exonum::encoding::F64;
    ///
    /// let wrapper = F64::new(1.0);
    /// let value = wrapper.get();
    /// ```
    pub fn get(&self) -> f64 {
        self.value
    }
}

impl<'a> Field<'a> for F32 {
    fn field_size() -> Offset {
        mem::size_of::<Self>() as Offset
    }

    unsafe fn read(buffer: &'a [u8], from: Offset, to: Offset) -> Self {
        Self::new(LittleEndian::read_f32(&buffer[from as usize..to as usize]))
    }

    fn write(&self, buffer: &mut Vec<u8>, from: Offset, to: Offset) {
        LittleEndian::write_f32(&mut buffer[from as usize..to as usize], self.get());
    }

    fn check(
        buffer: &'a [u8],
        from: CheckedOffset,
        to: CheckedOffset,
        latest_segment: CheckedOffset,
    ) -> EncodingResult {
        debug_assert_eq!((to - from)?.unchecked_offset(), Self::field_size());

        let from = from.unchecked_offset();
        let to = to.unchecked_offset();

        let value = LittleEndian::read_f32(&buffer[from as usize..to as usize]);
        match Self::try_from(value) {
            Some(_) => Ok(latest_segment),
            None => Err(EncodingError::UnsupportedFloat {
                position: from,
                value: f64::from(value),
            }),
        }
    }
}

impl<'a> Field<'a> for F64 {
    fn field_size() -> Offset {
        mem::size_of::<Self>() as Offset
    }

    unsafe fn read(buffer: &'a [u8], from: Offset, to: Offset) -> Self {
        Self::new(LittleEndian::read_f64(&buffer[from as usize..to as usize]))
    }

    fn write(&self, buffer: &mut Vec<u8>, from: Offset, to: Offset) {
        LittleEndian::write_f64(&mut buffer[from as usize..to as usize], self.get());
    }

    fn check(
        buffer: &'a [u8],
        from: CheckedOffset,
        to: CheckedOffset,
        latest_segment: CheckedOffset,
    ) -> EncodingResult {
        debug_assert_eq!((to - from)?.unchecked_offset(), Self::field_size());

        let from = from.unchecked_offset();
        let to = to.unchecked_offset();

        let value = LittleEndian::read_f64(&buffer[from as usize..to as usize]);
        match Self::try_from(value) {
            Some(_) => Ok(latest_segment),
            None => Err(EncodingError::UnsupportedFloat {
                position: from,
                value,
            }),
        }
    }
}

impl ExonumJson for F32 {
    fn deserialize_field<B: WriteBufferWrapper>(
        value: &Value,
        buffer: &mut B,
        from: Offset,
        to: Offset,
    ) -> Result<(), Box<Error>> {
        let number = value.as_f64().ok_or("Can't cast json as float")?;
        buffer.write(from, to, Self::new(number as f32));
        Ok(())
    }

    fn serialize_field(&self) -> Result<Value, Box<Error>> {
        Ok(Value::Number(
            Number::from_f64(f64::from(self.get())).ok_or(
                "Can't cast float as json",
            )?,
        ))
    }
}

#[cfg(feature="float_serialize")]
impl ExonumJson for F64 {
    fn deserialize_field<B: WriteBufferWrapper>(
        value: &Value,
        buffer: &mut B,
        from: Offset,
        to: Offset,
    ) -> Result<(), Box<Error>> {
        let number = value.as_f64().ok_or("Can't cast json as float")?;
        buffer.write(from, to, Self::new(number));
        Ok(())
    }

    fn serialize_field(&self) -> Result<Value, Box<Error>> {
        Ok(Value::Number(Number::from_f64(self.get()).ok_or(
            "Can't cast float as json",
        )?))
    }
}
