// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() { 
    
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_success() {
        #[cfg(feature = "pass")]
        { return;}  
        #[cfg(not(feature = "pass"))]  
        {  
             panic!("no cfg set"); // 你也可以选择在这里 panic!()，使构建失败  
        }  
      
    }
}
