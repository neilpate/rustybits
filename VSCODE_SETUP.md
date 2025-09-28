# VS Code Setup for Embedded Rust Development

This document explains how to configure VS Code for seamless embedded Rust development with the BBC micro:bit v2, including building, running, and debugging capabilities.

## VS Code Integration and rust-analyzer

### Code Lens Feature
When you open any example's `src/main.rs` file in VS Code, you'll notice small interactive buttons above the `#[entry]` function:

- **▶️ Run**: Executes `cargo run` and flashes to hardware
- **🐛 Debug**: Launches debugging session (requires debug panel)

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

## VS Code Workspace Configuration

Setting up VS Code for embedded Rust development requires several configuration files that work together to provide seamless building, running, and debugging capabilities.

### Project Structure
```
rustymicrobit/
├── .vscode/
│   ├── launch.json      # Debug configurations
│   ├── tasks.json       # Build and run tasks
│   └── settings.json    # Workspace settings (optional)
├── Embed.toml           # probe-rs configuration (shared)
├── example_01_hello_world/
│   ├── .cargo/
│   │   └── config.toml  # Example-specific cargo configuration
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml       # Example dependencies and metadata
│   ├── Cargo.lock       # Locked dependency versions
│   └── Embed.toml       # probe-rs configuration (per-example)
├── example_02_hello_world_minimal_dependencies/
│   ├── .cargo/
│   │   └── config.toml  # Example-specific cargo configuration
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml       # Example dependencies and metadata
│   ├── Cargo.lock       # Locked dependency versions
│   └── Embed.toml       # probe-rs configuration (per-example)
└── README.md            # Project overview
```

**Note**: This project uses **independent examples** rather than a Cargo workspace. Each example is a complete, standalone Rust project that can be copied and used independently.

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
        }
    ]
}
```

#### Key Task Properties:
- **`label`**: Name displayed in VS Code's task menu and referenced by other configurations
- **`type: "shell"`**: Executes commands in the system shell
- **`command` & `args`**: The actual command to run (equivalent to `cargo build` and `cargo run`)
- **`options.cwd`**: Working directory - crucial for finding local `.cargo/config.toml` which specifies the target
- **`group: "build"`**: Groups related tasks together
- **`problemMatcher: ["$rustc"]`**: Parses Rust compiler output to show errors in VS Code's Problems panel
- **`presentation`**: Controls how the terminal output is displayed

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

### Installation:
1. **Open VS Code Extensions** (Ctrl+Shift+X)
2. **Search for** each extension by name
3. **Click Install** for each extension

## Usage Workflows

### Development Workflow:
1. **Open** any example's `src/main.rs` file
2. **Click ▶️ Run** - Builds and flashes to micro:bit automatically
3. **Observe** program running on hardware
4. **Make changes** and repeat

### Debugging Workflow:
1. **Build first** - Click ▶️ Run to ensure fresh binary
2. **Open Debug Panel** - Press Ctrl+Shift+D
3. **Select Configuration** - Choose "Debug Example 01" or "Debug Example 02"
4. **Start Debugging** - Press F5 or click play button
5. **Set breakpoints** - Click in left margin of code
6. **Step through code** - Use debug controls

### Task Menu Workflow:
1. **Open Command Palette** - Press Ctrl+Shift+P
2. **Type** "Tasks: Run Task"
3. **Select** any available task (Build Example 01, etc.)
4. **Task runs** in integrated terminal

## Independent Examples Architecture

This project uses **independent examples** rather than a Cargo workspace:

- **Each example is self-contained**: Complete with its own `Cargo.toml`, `Cargo.lock`, and `.cargo/config.toml`
- **No workspace dependencies**: Examples can be copied and used independently
- **Reproducible builds**: Each `Cargo.lock` ensures identical dependency versions
- **Shared configuration**: VS Code settings and `Embed.toml` are shared across examples

## Integration Flow

### When you click ▶️ Run:

1. **rust-analyzer** detects the `#[entry]` function in the current file
2. **Looks up** the `"rust: cargo run"` task in `tasks.json`
3. **Executes** `cargo run` from the example directory (`${fileDirname}/..`)
4. **Cargo** uses the local `.cargo/config.toml` for runner and build settings
5. **Builds** the project with the ARM target using example-specific dependencies
6. **Runs** the configured runner: `probe-rs run --chip nRF52833_xxAA`
7. **probe-rs** uses `Embed.toml` configuration and flashes the program to micro:bit

### When you debug (Debug Panel → F5):

1. **VS Code** finds the selected launch configuration ("Debug Example 01" or "Debug Example 02")
2. **Runs** the `preLaunchTask` to build the project from the correct directory
3. **Launches** probe-rs in debug mode with `Embed.toml` settings
4. **Connects** to the micro:bit and loads the program with debug symbols
5. **Starts** the debug session with breakpoint support

This configuration provides a seamless embedded development experience where each example is completely independent while maintaining single-click build/flash/run capabilities!

## Troubleshooting

### Common Issues:

#### "Cannot find Cargo.toml" Error:
- **Cause**: Task running from wrong directory
- **Solution**: Make sure you have the example's `main.rs` file open when clicking ▶️ Run

#### Debug "Corrupted dump file" Error:
- **Cause**: VS Code trying to use wrong debugger
- **Solution**: Ensure probe-rs-debugger extension is installed and use Debug Panel (not Code Lens debug button)

#### Build Fails with Target Error:
- **Cause**: Missing or incorrect `.cargo/config.toml`
- **Solution**: Verify each example has its own `.cargo/config.toml` with correct target specification

#### probe-rs Connection Failed:
- **Cause**: micro:bit not connected or wrong chip specified
- **Solution**: Check USB connection and verify chip setting in launch.json matches your micro:bit version

### Tips:
- **Always open the specific example's `main.rs`** before running or debugging
- **Use the Debug Panel** for debugging rather than the Code Lens debug button
- **Check the terminal output** for detailed error messages
- **Ensure micro:bit is connected** before running or debugging