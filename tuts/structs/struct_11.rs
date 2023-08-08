//! Here, we are trying to implement this rust feature:
//! "The purpose of unit struct is to implement traits on it
//! and use that struct as value for the other traits'
//! associated type during implementation."
//!
//! In this code example, a coder would get bonus if has 'Blockchain' experience.
//!    and can design if has 'Fullstack' experience.

struct Coder;

#[derive(PartialEq)]
enum Experience {
    Backend,
    Blockchain,
    FullStack,
    Frontend,
}

trait Developer {
    type MyType;

    fn give_bonus(&self, working_style: Self::MyType) -> bool;
}

impl Developer for Coder {
    type MyType = Experience;

    fn give_bonus(&self, working_style: Self::MyType) -> bool {
        if working_style == Experience::Blockchain {
            return true;
        }
        false
    }
}

trait Designer {
    fn can_design(working_style: <Coder as Developer>::MyType) -> bool;
}

impl Designer for Coder {
    fn can_design(working_style: <Coder as Developer>::MyType) -> bool {
        if working_style == Experience::FullStack {
            println!("The coder can design as well");
            return true;
        }

        false
    }
}

#[test]
fn test_coder_design_capability() {
    let c1 = Coder;

    // coder shall receive bonus if Blockchain type
    assert_eq!(c1.give_bonus(Experience::Blockchain), true);
    assert_eq!(c1.give_bonus(Experience::Backend), false);
    assert_eq!(c1.give_bonus(Experience::Frontend), false);
    assert_eq!(c1.give_bonus(Experience::FullStack), false);

    // can design if Fullstack type
    assert_eq!(Coder::can_design(Experience::FullStack), true);
    assert_eq!(Coder::can_design(Experience::Backend), false);
    assert_eq!(Coder::can_design(Experience::Blockchain), false);
    assert_eq!(Coder::can_design(Experience::Frontend), false);
}
