# VHS Demo Script for cmd-keeper
# Run with: vhs demo.tape

# Configuration
Output demo.gif
Set FontSize 14
Set Width 1200
Set Height 800
Set Theme "Catppuccin Mocha"

# Start TUI
Type "cmd-keeper"
Enter
Sleep 2s

# Navigate through commands
Type "j"
Sleep 400ms
Type "j"
Sleep 400ms
Type "k"
Sleep 400ms

# Copy to clipboard
Type "y"
Sleep 1s

# Add new command
Type "a"
Sleep 1s
Type "docker compose up -d"
Sleep 600ms
Tab
Sleep 300ms
Type "Start all containers in background"
Sleep 600ms
Tab
Sleep 300ms
Type "docker,compose"
Sleep 500ms
Enter
Sleep 1.5s

# Navigate to the new command
Type "G"
Sleep 800ms

# Edit the command
Type "e"
Sleep 1s
Type "docker compose up -d --build"
Sleep 600ms
Tab
Sleep 300ms
Tab
Sleep 300ms
Enter
Sleep 1.5s

# Navigate up
Type "g"
Sleep 600ms

# Execute a command
Type "x"
Sleep 2s
Enter
Sleep 1.5s

# Navigate down
Type "j"
Sleep 400ms

# Delete confirmation (cancel)
Type "d"
Sleep 800ms
Type "n"
Sleep 800ms

# Quit
Type "q"
Sleep 500ms
