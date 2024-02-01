//! I am implementing the "Dungeons & Dragons" game as illustrated
//! in this talk, YT video: https://www.youtube.com/watch?v=grU-4u0Okto.
//! This talk is divided into 3 parts:
//! - Intro to traits
//! - trait bounds
//! - trait object
//!
//! ==== Part 1 =====
//!
//! The characters have many characteristics/traits:
//! - Strength
//! - Intelligence
//! - Dexterity
//! - Wisdom
//! - Constitution bonus: means adding it's value `x` to the random dice roll result.
//! - Charisma
//!
//! Firstly, the focus is onto constitution bonus where most of the characters have
//! constitution bonus as `0`.
//!
//! ==== Part 2 =====
//!
//! Then, we add another character [`HalfElf`]
//!
//! Both `Elf`, `HalfElf` speaks same language i.e. Elvish. But `Dwarf` speaks `Dwarvish`.
//!
//! Let's create a function: `speak_elvish()` that accepts type which has `Elvish` trait
//! implemented.
//!
//! **Summary**: Trait bounds allows a function to accept types that
//! has implemented certain trait(s).
//!
//! ==== Part 3 =====
//!
//! Now, there is a wizard which needs to run 4 spells:
//!
//! - Cantrip
//! - Enchantment
//! - Transmutation
//! - Necromancy
//!
//! All need to be casted which is the common behavior among these.
//!
//! Also, need to store these spells in a spellbook. So, how to do these?
//! Especially when they are of different type.
//!
//! And that's where `trait-object` comes into the picture to help you get
//! this done with hetrogenous collection.
//!
//! Code: "./dungeons_dragons_to.rs"

// === Characters

struct Dwarf {
	name: String,
}
struct Elf {
	name: String,
}
struct HalfOrc {
	name: String,
}
struct Human {
	name: String,
}

// added another character
struct HalfElf {
	name: String,
}

// added later

trait Constitution {
	fn constitution_bonus(&self) -> u8 {
		0
	}
}
// as most of the characters
impl Constitution for Dwarf {}
impl Constitution for Elf {}
impl Constitution for HalfOrc {}
impl Constitution for Human {}

// create a common trait for language spoken by
// characters: `Elf`, `HalfElf`
trait Elvish {}

//
impl Elvish for Elf {}
impl Elvish for HalfElf {}

/// a function that is common among characters
fn speak_elvish<T: Elvish>(_character: T) -> &'static str {
	"Elvish"
}

pub(crate) fn main() {
	/* part-1 */
	let my_human = Human { name: "Alice".to_string() };
	assert_eq!(my_human.constitution_bonus(), 0);

	/* part-2 */
	let my_elf = Elf { name: "NellElf".to_string() };
	assert_eq!(speak_elvish(my_elf), "Elvish");
	let my_half_elf = HalfElf { name: "NellElfHalf".to_string() };
	assert_eq!(speak_elvish(my_half_elf), "Elvish");

	// throws error ❌ as `Elvish` trait is not implemented for `Dwarf`.
	// Because they speak `Dwavish`, not Elvish lang.
	let my_dwarf = Dwarf { name: "Dwarf1".to_string() };
	// speak_elvish(my_dwarf);
}
