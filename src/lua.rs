use mlua::prelude::*;
use std::thread;
use std::time::Duration;

use std::sync::LazyLock;
use std::sync::Mutex;

static TICK_INTERVAL: Duration = Duration::from_millis(16); // 1/60s
pub static STOP_SIGNAL: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

pub static NUMBER_OUTPUT_TABLE: LazyLock<Mutex<[f64; 32]>> =
    LazyLock::new(|| Mutex::new([0.0; 32]));
pub static NUMBER_INPUT_TABLE: LazyLock<Mutex<[f64; 32]>> = LazyLock::new(|| Mutex::new([0.0; 32]));

pub static BOOL_OUTPUT_TABLE: LazyLock<Mutex<[bool; 32]>> =
    LazyLock::new(|| Mutex::new([false; 32]));
pub static BOOL_INPUT_TABLE: LazyLock<Mutex<[bool; 32]>> =
    LazyLock::new(|| Mutex::new([false; 32]));

// Function to simulate a microcontroller environment
pub fn run_microcontroller(script: &str) -> LuaResult<()> {
    // Lua with sandboxing
    let lua = Lua::new_with(mlua::StdLib::NONE, mlua::LuaOptions::default())?;

    // Output functions
    let output = lua.create_table()?;

    output.set(
        "setNumber",
        lua.create_function(|_, (index, value): (f64, f64)| {
            println!("Output {}: {}", index, value);
            NUMBER_OUTPUT_TABLE.lock().unwrap()[index as usize] = value; // TODO: Fix index out of bounds and on others too
            Ok(())
        })?,
    )?;

    output.set(
        "setBool",
        lua.create_function(|_, (index, value): (f64, bool)| {
            println!("Output {}: {}", index, value);
            BOOL_OUTPUT_TABLE.lock().unwrap()[index as usize] = value;
            Ok(())
        })?,
    )?;

    lua.globals().set("output", output)?;

    // Input functions
    let input = lua.create_table()?;

    input.set(
        "getNumber",
        lua.create_function(|_, index: f64| {
            println!("Input {}", index); // For debugging purposes
            Ok(NUMBER_INPUT_TABLE
                .lock()
                .unwrap()
                .get(index as usize)
                .copied()
                .unwrap_or(0.0))
        })?,
    )?;

    input.set(
        "getBool",
        lua.create_function(|_, index: f64| {
            println!("Input {}", index); // For debugging purposes
            Ok(BOOL_INPUT_TABLE
                .lock()
                .unwrap()
                .get(index as usize)
                .copied()
                .unwrap_or(false))
        })?,
    )?;

    lua.globals().set("input", input)?;

    // TODO: Properties, Drawing, HTTP

    lua.load(script).exec()?;

    // Get functions
    let on_tick: LuaResult<LuaFunction> = lua.globals().get("onTick");
    let on_draw: LuaResult<LuaFunction> = lua.globals().get("onDraw");

    loop {
        // Run tick function if defined
        if let Ok(on_tick) = &on_tick {
            on_tick.call::<()>(())?;
        }

        // Run draw function if defined
        if let Ok(on_draw) = &on_draw {
            on_draw.call::<()>(())?;
        }

        thread::sleep(TICK_INTERVAL); // Run at correct speed

        // Stop if another thread requests it
        if *STOP_SIGNAL.lock().unwrap() {
            break;
        }
    }

    // Reset the stop signal for the next run
    *STOP_SIGNAL.lock().unwrap() = false;

    Ok(())
}
