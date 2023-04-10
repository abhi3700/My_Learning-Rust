#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::fmt::{self, Debug, Display};
trait Details: Debug {
    fn summary(&self) -> String;
}
struct House {
    area_sqft: u32,
    purchased_date: String,
}
#[automatically_derived]
impl ::core::default::Default for House {
    #[inline]
    fn default() -> House {
        House {
            area_sqft: ::core::default::Default::default(),
            purchased_date: ::core::default::Default::default(),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for House {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "House",
            "area_sqft",
            &self.area_sqft,
            "purchased_date",
            &&self.purchased_date,
        )
    }
}
struct Vehicle {
    name: String,
    model: String,
    purchased_date: String,
}
#[automatically_derived]
impl ::core::default::Default for Vehicle {
    #[inline]
    fn default() -> Vehicle {
        Vehicle {
            name: ::core::default::Default::default(),
            model: ::core::default::Default::default(),
            purchased_date: ::core::default::Default::default(),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Vehicle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Vehicle",
            "name",
            &self.name,
            "model",
            &self.model,
            "purchased_date",
            &&self.purchased_date,
        )
    }
}
impl Details for House {
    fn summary(&self) -> String {
        {
            let res = ::alloc::fmt::format(
                format_args!(
                    "The House that was purchased on {0} has an area of {1}", self
                    .purchased_date, self.area_sqft
                ),
            );
            res
        }
    }
}
impl Details for Vehicle {
    fn summary(&self) -> String {
        {
            let res = ::alloc::fmt::format(
                format_args!(
                    "The vehicle with name {0}, model {1} is purchased on {2}", self
                    .name, self.model, self.purchased_date
                ),
            );
            res
        }
    }
}
fn foo(flag: bool) -> Box<dyn Details> {
    if flag { Box::new(House::default()) } else { Box::new(Vehicle::default()) }
}
impl Display for dyn Details {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{0}", self.summary()))
    }
}
fn main() {
    {
        ::std::io::_print(format_args!("{0}\n", foo(true)));
    };
    {
        ::std::io::_print(format_args!("{0:?}\n", foo(true)));
    };
}
