use mlua::prelude::*;

// Simulate a microcontroller environment
pub fn run_microcontroller(script: &str) -> LuaResult<()> {
    println!("Running script: {}", script);
    let lua = Lua::new_with(mlua::StdLib::NONE, mlua::LuaOptions::default())?;

    lua.load(script).exec()?;

    let test: LuaFunction = lua.globals().get("test")?;
    let result = test.call::<()>(())?;

    Ok(())
}
