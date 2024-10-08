// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.


#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // Removed the unnecessary unwrap() call
        println!("my_option is None.");
    }

    let my_arr = &[
        -1, -2, -3, // Added a comma here
        -4, -5, -6, // Added a comma here
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // Initialize an empty Vec directly
    let my_empty_vec: Vec<i32> = Vec::new(); 
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a, &mut value_b); // Use swap function to swap values
    println!("value a: {}; value b: {}", value_a, value_b);
}
