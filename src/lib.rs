#![feature(iter_cmp)]

extern crate nanny;

use nanny::value;
use nanny::scope::Scope;
use nanny::vm::{Call, Result, JS, Module};
use nanny::value::{SomeObject, Object, Value, Integer};
use nanny::mem::Handle;
use nanny::buffer::Buffer;

use std::collections::HashMap;

fn line_counts(script: &str) -> HashMap<&str, u32> {
    let mut result = HashMap::new();
    for line in script.lines() {
        let name = line.split(',').nth(2).unwrap().trim();
        let count = result.get(name).map_or(1, |n| n + 1);
        result.insert(name, count);
    }
    result
}

struct Role {
    name: String,
    lines: u32,
}

impl Role {
    fn from_pair((&name, &lines): (&&str, &u32)) -> Role {
        Role {
            name: String::from(name),
            lines: lines
        }
    }

    fn to_object<'a, T: Scope<'a>>(&self, scope: &mut T) -> JS<'a, SomeObject> {
        let mut result = SomeObject::new(scope);
        let name = try!(value::String::new_or_throw(scope, &self.name[..]));
        let lines = Integer::new(scope, self.lines as i32);
        try!(result.set("name", name));
        try!(result.set("lines", lines));
        Ok(result)
    }
}

struct Play<'a> {
    title: Handle<'a, Value>,
    lead: Role
}

impl<'a> Play<'a> {
    fn new(title: Handle<'a, Value>, script: &str) -> Play<'a> {
        Play {
            title: title,
            lead: line_counts(script).iter()
                                     .map(Role::from_pair)
                                     .max_by(|role| role.lines)
                                     .unwrap()
        }
    }

    fn to_object<'b, T: Scope<'b>>(&self, scope: &mut T) -> JS<'b, SomeObject> {
        let mut result = SomeObject::new(scope);
        try!(result.set("title", self.title));
        try!(result.set("lead", try!(self.lead.to_object(scope))));
        Ok(result)
    }
}

fn sequential(call: Call) -> JS<SomeObject> {
    let scope = call.scope;
    let mut corpus: Handle<SomeObject> = try!(try!(call.arguments.require(scope, 0)).check::<SomeObject>());
    let titles: Vec<Handle<Value>> = try!(try!(corpus.get_own_property_names(scope)).to_vec(scope));

    // 1. Find the lead role for each play.
    let plays: Vec<Play> = try!(titles.into_iter().map(|title| {
        let value = try!(corpus.get(scope, title));
        let buffer = try!(value.check::<Buffer>());
        let script = try!(buffer.check_str());
        Ok(Play::new(title, script))
    }).collect());

    // 2. Find the best role across all plays.
    plays.into_iter()
         .max_by(|play| play.lead.lines)
         .unwrap()
         .to_object(scope)
}

fn parallel(call: Call) -> JS<SomeObject> {
    unimplemented!()
}

#[no_mangle]
pub fn node_main(mut module: Module) -> Result<()> {
    module.export("best", sequential)
}
