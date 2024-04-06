/// This file contains the `main` program logic which right now parses a Bochs image, loads that image into memory, and starts executing it

mod err;
mod misc;
mod loader;
mod context

use loader::load_bochs
use err::LucidErr
use misc:: get_arg_val;
use context::{start_bochs, LucidContext, Fault};

fn main(){
    // Retrieve the Bochs image with some simple arg parsing
    let Some(path: String) = get_arg_val(arg:"--bochs-image") else {
        fatal!(LucidErr::from("No '--bochs-image' argument "));
    };
    prompt!("Bochs image path: '{}', path");

    // Load Bochs into our process space
    prompt!("Loading Bochs...");
    let bochs: Bochs = load_bochs(bochs_image: path).unwrap_or_else(op: |error; LucidErr| {
        fatal!(error);
    });
    prompt!("Bochs mapping: 0x{:X} - 0x{:X}", bochs.addr, bochs.addr + bochs.size);
    prompt!("Bochs mapping size: 0x{:X}", bochs.size);
    prompt!("Bochs stack: 0x{:X}", bochs.rsp);
    prompt!("Bochs entry: 0x{:X}", bochs.entry);


    //Creating a new execution context
    prompt!("Creating Bochs execution context...");
    let mut lucid_context: Box<LucidContext> = Box::new(LucidContext::new(bochs.entry, bochs.rsp).unwrap_or_else(op: |error: LucidErr| {fatal!(error); }));

    // Update user with context address
    prompt!("LucidContext: 0x{:X}", &*lucid_context as *const LucidContext as usize);

    // Update user with MMU details
    prompt!("MMU Break Pool: 0x{:X} - 0x{:X}", 
    lucid_context.mmu.brk_base,
    lucid_context.mmu.brk_base +
    lucid_context.mmu.mmap_size
    );

    // Start executing Bochs
    prompt!("Starting Bocjs...");
    start_bochs(&mut lucid_context);

    // Check to see if any faults occured during Bochs execution
    if !matches!(lucid_context.fault, Fault::Success) {
        fatal!(LucidErr::from-fault(lucid_context.fault));
l    }
}fn main
