# VS Code Setup for Embedded Rust Development

This document explains how to configure VS Code for seamless embedded Rust development with the BBC micro:bit v2, including building, running, and debugging capabilities.

## VS Code Integration and rust-analyzer

### Code Lens Feature
When you open any example's `src/main.rs` file in VS Code, you'll notice small interactive buttons above the `#[entry]` function:

- **▶️ Run**: Executes `cargo run` and flashes to hardware
- **🐛 Debug**: ⚠️ **Does not work for embedded projects** - use the Debug Panel instead (F5 or Run & Debug sidebar)

### How Code Lens Works
The **rust-analyzer** Language Server analyzes your code and provides these "Code Lens" actions by:

1. **Detecting entry points**: Recognizes `#[entry]` as a runnable binary target
2. **Reading Cargo.toml**: Understands your project's binary configuration
3. **Generating commands**: Creates the appropriate `cargo run` command
4. **Integration with runner**: Uses your `.cargo/config.toml` runner configuration

### What Happens When You Click Run
When you click the ▶️ Run button, rust-analyzer executes `cargo run` which:

1. **Builds** the project with the ARM target (from `.cargo/config.toml`)
2. **Uses** the configured runner: `probe-rs run --chip nRF52833_xxAA`
3. **Flashes** the program to the micro:bit
4. **Runs** the program on the hardware

This seamlessly integrates the entire compilation → linking → flashing pipeline into a single click.

### Debugging Workflow

**Important**: The 🐛 Debug button in Code Lens does **not** work for embedded projects. Instead, use the proper debugging workflow:

#### To Start Debugging:
1. **Open the Debug Panel**: Click the "Run and Debug" icon in the sidebar (or press `Ctrl+Shift+D`)
2. **Select Configuration**: Choose "Debug Example 01", "Debug Example 02", or "Debug Example 03" from the dropdown
3. **Start Debugging**: Click the green play button or press `F5`

#### What Happens During Debug:
1. **Pre-build**: Runs the configured `preLaunchTask` to build the project
2. **Flash with Debug Info**: probe-rs flashes the program with debug symbols
3. **Attach Debugger**: Connects GDB-compatible debugger to the running hardware
4. **Debug Interface**: VS Code opens debugging panels (variables, call stack, breakpoints)

#### Debugging Features Available:
- **Breakpoints**: Click in the margin to pause execution at specific lines
- **Step Execution**: Step over, step into, step out of functions
- **Variable Inspection**: View and modify variable values in real-time
- **Memory View**: Examine raw memory and processor registers
- **Call Stack**: See the function call hierarchy

### Visual Guide

#### Code Lens Run Button
When viewing the source code, you'll see a small ▶️ "Run" arrow above the `#[entry]` function. This is provided by rust-analyzer and lets you run the example with a single click!

<img width="1660" height="773" alt="VS Code Run button in source code" src="https://github.com/user-attachments/assets/744fbe24-fdd4-4cfb-af37-8be0536d5d28" />

#### Debug Session Interface
You can debug examples directly on the micro:bit hardware. Start the session using the Debug Panel (F5):

<img width="2257" height="1084" alt="VS Code debugging session" src="https://github.com/user-attachments/assets/84128dfc-99a1-4703-adae-b770a1a1c9fa" />

The debug configuration is set up to halt the CPU on load. You can resume execution by pressing the run arrow at the top.

#### Breakpoint Debugging
Use breakpoints to pause execution and inspect program state:

<img width="1633" height="800" alt="VS Code breakpoint debugging" src="https://github.com/user-attachments/assets/25fded61-5eb7-4921-a8c8-90032c17cc9f" />

## VS Code Workspace Configuration

Setting up VS Code for embedded Rust development requires several configuration files that work together to provide seamless building, running, and debugging capabilities.

### VS Code Configuration Files Location
```
.vscode/
├── tasks.json       # Build and run tasks
├── launch.json      # Debug configurations  
└── settings.json    # Workspace settings (optional)
```

## Configuration Files

### Tasks Configuration (`.vscode/tasks.json`)

Tasks define how VS Code executes build and run operations:

```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "rust: cargo build",
            "type": "shell",
            "command": "cargo",
            "args": [
                "build"
            ],
            "options": {
                "cwd": "${fileDirname}/.."
            },
            "group": "build",
            "presentation": {
                "reveal": "always",
                "panel": "new"
            },
            "problemMatcher": [
                "$rustc"
            ]
        },
        {
            "label": "rust: cargo run",
            "type": "shell",
            "command": "cargo",
            "args": [
                "run"
            ],
            "options": {
                "cwd": "${fileDirname}/.."
            },
            "group": "build",
            "presentation": {
                "reveal": "always",
                "panel": "new"
            },
            "problemMatcher": [
                "$rustc"
            ]
        },
        {
            "label": "Build Example 01",
            "type": "shell",
            "command": "cargo",
            "args": [
                "build"
            ],
            "options": {
                "cwd": "${workspaceFolder}/example_01_hello_world"
            },
            "group": "build",
            "problemMatcher": [
                "$rustc"
            ]
        },
        {
            "label": "Build Example 02",
            "type": "shell",
            "command": "cargo",
            "args": [
                "build"
            ],
            "options": {
                "cwd": "${workspaceFolder}/example_02_hello_world_minimal_dependencies"
            },
            "group": "build",
            "problemMatcher": [
                "$rustc"
            ]
        },
        {
            "label": "Build Example 03",
            "type": "shell",
            "command": "cargo",
            "args": [
                "build"
            ],
            "options": {
                "cwd": "${workspaceFolder}/example_03_hello_world_no_dependencies"
            },
            "group": "build",
            "problemMatcher": [
                "$rustc"
            ]
        }
    ]
}
```

#### Special Tasks:
- **`"rust: cargo build"`** and **`"rust: cargo run"`**: Override rust-analyzer's built-in tasks
- This ensures that when you click the ▶️ Run button, it uses our configuration with the correct working directory

#### Target Configuration:
Notice that the tasks don't explicitly specify `--target thumbv7em-none-eabihf`. This is because the target is configured in each example's `.cargo/config.toml` file, making the tasks simpler and ensuring consistency.

### Launch Configuration (`.vscode/launch.json`)

Launch configurations define debugging sessions:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "probe-rs-debug",
            "request": "launch",
            "name": "Debug Example 01",
            "cwd": "${workspaceFolder}/example_01_hello_world",
            "connectUnderReset": false,
            "chip": "nRF52833_xxAA",
            "flashingConfig": {
                "flashingEnabled": true,
                "haltAfterReset": false
            },
            "coreConfigs": [
                {
                    "coreIndex": 0,
                    "programBinary": "target/thumbv7em-none-eabihf/debug/example_01_hello_world"
                }
            ],
            "preLaunchTask": "Build Example 01"
        },
        {
            "type": "probe-rs-debug",
            "request": "launch",
            "name": "Debug Example 02",
            "cwd": "${workspaceFolder}/example_02_hello_world_minimal_dependencies",
            "connectUnderReset": false,
            "chip": "nRF52833_xxAA",
            "flashingConfig": {
                "flashingEnabled": true,
                "haltAfterReset": false
            },
            "coreConfigs": [
                {
                    "coreIndex": 0,
                    "programBinary": "target/thumbv7em-none-eabihf/debug/example_02_hello_world_minimal_dependencies"
                }
            ],
            "preLaunchTask": "Build Example 02"
        },
        {
            "type": "probe-rs-debug",
            "request": "launch",
            "name": "Debug Example 03",
            "cwd": "${workspaceFolder}/example_03_hello_world_no_dependencies",
            "connectUnderReset": false,
            "chip": "nRF52833_xxAA",
            "flashingConfig": {
                "flashingEnabled": true,
                "haltAfterReset": false
            },
            "coreConfigs": [
                {
                    "coreIndex": 0,
                    "programBinary": "target/thumbv7em-none-eabihf/debug/main"
                }
            ],
            "preLaunchTask": "Build Example 03"
        }
    ]
}
```

#### Key Launch Properties:
- **`type: "probe-rs-debug"`**: Uses the probe-rs VS Code extension for ARM debugging
- **`request: "launch"`**: Starts a new debugging session (vs "attach" to existing)
- **`cwd`**: Working directory for the debug session
- **`chip: "nRF52833_xxAA"`**: Specific microcontroller target for probe-rs
- **`flashingConfig`**: Controls how the program is loaded onto the microcontroller
- **`programBinary`**: Path to the compiled ELF file to debug
- **`preLaunchTask`**: Task to run before debugging (builds the project)

### VS Code Settings (`.vscode/settings.json`) - Optional

Workspace-specific settings can enhance the development experience:

```json
{
    "rust-analyzer.cargo.features": "all"
}
```

#### Setting Explanations:
- **`rust-analyzer.cargo.features: "all"`**: Enables all available Cargo features during code analysis, providing complete IntelliSense and error checking for all feature-gated code

## Required Extensions

To use this setup, you need these VS Code extensions:

### Essential Extensions:
1. **rust-analyzer** (`rust-lang.rust-analyzer`)
   - Provides Rust language support, Code Lens, and IntelliSense
   - Enables the ▶️ Run and 🐛 Debug buttons

2. **probe-rs-debugger** (`probe-rs.probe-rs-debugger`) 
   - Provides embedded debugging support for ARM microcontrollers
   - Required for the debug configurations to work
