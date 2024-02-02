//! Continued from "./dungeons_dragons.rs"...

struct Cantrip;
struct Enchantment;
struct Transmutation;
struct Necromancy;

trait Cast {
	fn cast(&self);
}

impl Cast for Cantrip {
	fn cast(&self) {
		println!("\tspell ran for Cantrip");
	}
}
impl Cast for Enchantment {
	fn cast(&self) {
		// logic
		println!("\tspell ran for Enchantment");
	}
}
impl Cast for Transmutation {
	fn cast(&self) {
		// logic
		println!("\tspell ran for Transmutation");
	}
}
impl Cast for Necromancy {
	fn cast(&self) {
		// logic
		println!("\tspell ran for Necromancy");
	}
}

/// let's have a collection - spellbook which stores all the spells
/// All the types that has `Cast` trait implemented.
/// NOTE: `impl Cast` or generic could have been used instead of `dyn Trait`
/// so as to remove the runtime cost as everything is known at compile time.
///
/// But, there would have been a lot of code duplication. Therefore, to reduce
/// the code size, `dyn Cast` is used.
///
/// Summary:
/// - `impl Trait` or Generic has although more code size, but has no runtime cost.
/// - `dyn Trait` is used to have lesser code size, but has runtime cost.
struct SpellBook {
	spells: Vec<Box<dyn Cast>>,
}

impl SpellBook {
	fn run(&self) {
		for spell in self.spells.iter() {
			// when this function is called, `vtable` is searched
			// to get the corresponding function pointer so as to
			// execute the corresponding code.
			// You can find the individual `cast` implementation
			// for respective spells above in `impl` block.
			spell.cast();
		}
	}
}

pub(crate) fn main() {
	let my_spellbook = SpellBook {
		spells: vec![
			Box::new(Cantrip),
			Box::new(Enchantment),
			Box::new(Transmutation),
			Box::new(Necromancy),
		],
	};

	// Here, wizard runs all the spells in one go.
	println!("Wizard is running all the spells... 🪄✨");
	my_spellbook.run();
}
