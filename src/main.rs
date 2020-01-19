pub mod piaic_batch1
{
    pub mod batch1
    {
        pub fn karachi()
        {
            println!("PIAIC Batch-1 is in Karachi!");
        }
    }
}

// for lib.rs
mod lib;

// for separate library package 
use piaic_batch3;

fn main() 
{
    crate::piaic_batch1::batch1::karachi();

    // lib.rs
    lib::piaic_batch2::batch2::khi();

    // for separate library package
    piaic_batch3::piaic_batch3mod::batch3::islamabad(); 
}
