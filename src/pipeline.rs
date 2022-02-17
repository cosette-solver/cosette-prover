use std::cell::RefCell;
use std::collections::HashMap;

use imbl::vector;
use z3::{Config, Context, Solver};
use {normal as nom, syntax as syn};

use crate::pipeline::normal::StbEnv;
use crate::pipeline::shared::{Eval, Schema};
use crate::pipeline::unify::{Unify, UnifyEnv};

pub mod normal;
mod null;
pub mod partial;
pub mod relation;
pub mod shared;
pub mod syntax;
#[cfg(test)]
mod tests;
pub mod unify;

pub fn evaluate(rel: syn::Relation, schemas: &[Schema]) -> nom::Relation {
	log::info!("Syntax:\n{}", rel);
	let prt = (&partial::Env::default()).eval(rel);
	let nom = (0, schemas).eval(prt);
	log::info!("Normal:\n{}", nom);
	let mut config = Config::new();
	config.set_timeout_msec(2000);
	let ctx = &Context::new(&config);
	let solver = &Solver::new(ctx);
	let uexpr_subst = &vector![];
	let z3_subst = &vector![];
	let h_ops = &RefCell::new(HashMap::new());
	let rel_h_ops = &RefCell::new(HashMap::new());
	let env = StbEnv::new(uexpr_subst, 0, solver, z3_subst, h_ops, rel_h_ops);
	let stb = env.eval(nom);
	log::info!("Stable:\n{}", stb);
	stb
}

pub fn unify(rel1: nom::Relation, rel2: nom::Relation) -> bool {
	let mut config = Config::new();
	config.set_timeout_msec(5000);
	let ctx = &Context::new(&config);
	let solver = &Solver::new(ctx);
	let subst1 = &vector![];
	let subst2 = &vector![];
	UnifyEnv::new(solver, subst1, subst2).unify(&rel1, &rel2)
}