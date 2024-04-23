mod getting_started;
use getting_started::run1;

mod primitives;
use primitives::run2;

mod use_type;
use use_type::run3;

mod variable_binding;
use variable_binding::run4;

fn main() {
    run1();
    run2();
    run3();
    run4();
}
