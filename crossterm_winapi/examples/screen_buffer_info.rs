extern crate crossterm_winapi;

use crossterm_winapi::{Handle, HandleType};

fn main() {
    /// see the description of the types to see what they do.
    let out_put_handle = Handle::new(HandleType::OutputHandle).unwrap();
    let out_put_handle = Handle::new(HandleType::InputHandle).unwrap();
    let curr_out_put_handle = Handle::new(HandleType::CurrentOutputHandle).unwrap();
    let curr_out_put_handle = Handle::new(HandleType::CurrentInputHandle).unwrap();

    // now you have this handle you might want to get the WinApi `HANDLE` it is wrapping.
    // you can do this by defencing.
    let handle = *out_put_handle;
    //    let handle = Handle::from(/* winapi::um::winnt::HANDLE */);
}
