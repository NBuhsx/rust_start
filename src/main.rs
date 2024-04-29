mod getting_started;
use getting_started::run1;

mod primitives;
use primitives::run2;

mod use_type;
use use_type::run3;

mod variable_binding;
use variable_binding::run4;

mod types;
use types::run5;

mod conversion;
use conversion::run6;

mod expressions;
use expressions::run7;

mod loop_control;
use loop_control::run8;

fn main() {
    run1();
    run2();
    run3();
    run4();
    run5();
    run6();
    run7();
    run8();
}
