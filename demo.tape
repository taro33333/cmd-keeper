# VHS Demo Script for cmd-keeper
# Run with: vhs demo.tape

# Configuration
Output demo.gif
Set FontSize 14
Set Width 1200
Set Height 800
Set Theme "Catppuccin Mocha"

# Start
Type "cmd-keeper"
Enter
Sleep 2s

# Navigate down
Type "j"
Sleep 500ms
Type "j"
Sleep 500ms

# Navigate up
Type "k"
Sleep 500ms

# Copy to clipboard
Type "y"
Sleep 1s

# Add new command
Type "a"
Sleep 1s

# Type command
Type "docker compose up -d"
Sleep 500ms
Tab
Sleep 300ms

# Type description
Type "Start all containers in background"
Sleep 500ms
Tab
Sleep 300ms

# Type tags
Type "docker,compose"
Sleep 500ms

# Save with Enter (in Tags field, Enter saves)
Enter
Sleep 1s

# Show the new command
Type "G"
Sleep 1s

# Quit
Type "q"
Sleep 500ms
