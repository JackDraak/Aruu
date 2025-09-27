# Testing cargo run vs direct execution

## Test Procedure

### 1. Test with cargo run:
```bash
cd /home/jdraak/Aruu/aruu
cargo run
```
**Expected**: Red and blue overlays visible + debug messages every second

### 2. Test with direct execution from project directory:
```bash
cd /home/jdraak/Aruu/aruu
./target/debug/aruu
```
**Expected**: Should work identically to cargo run

### 3. Test with direct execution from different directory:
```bash
cd /home/jdraak
./Aruu/aruu/target/debug/aruu
```
**Expected**: Might fail if there are working directory dependencies

## What to Look For:

1. **Console Messages**: Look for these debug messages appearing every second:
   ```
   🔍 EnhancedFrameComposer: debug=true, control=true
   🎨 OverlaySystem::render - 2 enabled overlays, total: 2
   ```

2. **Error Messages**: Look for any error messages starting with:
   ```
   ❌ Overlay rendering error: ...
   ```

3. **Visual Overlays**:
   - Red rectangle on right side
   - Blue rectangle on top-left

## Debugging Steps:

If direct execution doesn't work:

1. **Check if debug messages appear**: This tells us if the overlay system is being called
2. **Check for error messages**: This tells us if rendering is failing
3. **Try from different directories**: This tells us if it's a working directory issue

## What Each Result Means:

- **No debug messages**: Overlay system not initialized or crashing early
- **Debug messages but no visuals**: Shader compilation or rendering issue
- **Works from project dir but not others**: Working directory dependency
- **Error messages**: Specific technical issue we can fix