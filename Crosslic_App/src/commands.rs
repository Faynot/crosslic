use crosslic::command;

//===================
//      Commands
//===================

// Simple command examples

#[command]
pub fn sum(a: i32, b: i32) -> Result<i32, String> {
    Ok(a + b)
}

#[command]
pub fn greet(name: String) -> Result<String, String> {
    Ok(format!("Hello, {}!", name))
}
