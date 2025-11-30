#[warn(unused_imports)]

mod vectors_practice;
mod arrays_practice;
mod slices_practice;
mod contact_management;
mod safe_data_sharing;
mod combine_arc_mutex;

use crate::vectors_practice::{vectors_practice};
use crate::arrays_practice::{arrays_practice};
use crate::combine_arc_mutex::run_combine_arc_mutex;
use crate::contact_management::run_contact_management;
use crate::safe_data_sharing::run_safe_data_sharing;
use crate::slices_practice::{slices_practice};

fn main() {
    // vectors_practice();
    // arrays_practice();
    // slices_practice();
    // run_contact_management();
    // run_safe_data_sharing();
    run_combine_arc_mutex();
}
