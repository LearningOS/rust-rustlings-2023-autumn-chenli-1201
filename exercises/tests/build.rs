//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
fn main() {  
    // In tests7, we should set up an environment variable  
    // called `TEST_FOO`. Print in the standard output to let  
    // Cargo do it.  
    let timestamp = std::time::SystemTime::now()  
        .duration_since(std::time::UNIX_EPOCH)  
        .unwrap()  
        .as_secs();  
    let your_command1 = format!("TEST_FOO={}", timestamp);  
    println!("cargo:rustc-env={}", your_command1);  
     // In tests8, we should enable "pass" feature to make the    
        // testcase return early. Fill in the command to tell    
        // Cargo about that.    
        #[cfg(feature = "pass")]{
        println!("cargo:rustc-cfg=pass"); } 
        #[cfg(not(feature = "pass"))]  
        {  
             panic!("no cfg set"); // 你也可以选择在这里 panic!()，使构建失败  
        }   }