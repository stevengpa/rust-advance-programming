#[warn(unused_imports)]

mod vectors_practice;
mod arrays_practice;
mod slices_practice;
mod contact_management;
mod safe_data_sharing;
mod combine_arc_mutex;
mod counter_atomic;
mod thread_channel;
mod thread_channels_mutex;
mod thread_rwlock;
mod trait_implementations;
mod trait_image_processing;

use crate::vectors_practice::{vectors_practice};
use crate::arrays_practice::{arrays_practice};
use crate::combine_arc_mutex::run_combine_arc_mutex;
use crate::contact_management::run_contact_management;
use crate::counter_atomic::run_counter_atomic;
use crate::safe_data_sharing::run_safe_data_sharing;
use crate::slices_practice::{slices_practice};
use crate::thread_channel::run_thread_channels;
use crate::thread_channels_mutex::run_thread_channels_mutex;
use crate::thread_rwlock::run_thread_rwlock;
use crate::trait_image_processing::run_image_processing;

fn main() {
    // vectors_practice();
    // arrays_practice();
    // slices_practice();
    // run_contact_management();
    // run_safe_data_sharing();
    // run_combine_arc_mutex();
    // run_counter_atomic();
    // run_thread_channels();
    // run_thread_channels_mutex();
    // run_thread_rwlock();
    // run_image_processing();
}
