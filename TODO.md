# TODO: Fixing Build Errors and Completing the Refactor

## Completed Tasks ✓

### 1. Fix `draw/info.rs` file ✓
- Update `display_info()` method to accept color parameters

### 2. Fix `draw/game_state.rs` file ✓
- Fix the `display_game_state()` method to properly pass color parameters to `display_info()`

### 3. Fix `tui.rs` Color implementation issues ✓
- Ensure all color types (LightWhite, Black, Blue, LightGreen, LightYellow, Reset) properly implement the `Color` trait
- Check if the color types are properly imported and defined

### 4. Fix WebStdout and MockInput implementation issues in tests ✓
- Ensure `MockInput` implements `Default` trait
- Fix the test in `tui.rs` that uses the mock implementations

### 5. Fix Web implementation dependencies ✓
- Add `solitext_core` as a dependency to the `solitext-web` crate

### 6. Verify integrations between modules (Partially Done)
- Ensure that the generic trait bounds are consistent across all modules
- Check that all generic parameters are properly constrained 

### 7. Additional testing (Basic Tests Done)
- Created simple tests for both implementations

### 8. Update documentation ✓
- Document the new generic interfaces
- Update existing documentation to reflect the new architecture

## Remaining Tasks

### 1. Fix trait bound constraints in Draw and implementations
- There are several trait bounds that need to be fixed:
  - Add `where <T as Terminal>::RawTerminal: Write` constraint to implementations in info.rs
  - Fix impl blocks in all Draw struct implementations to include the proper constraints
  - Ensure the `Write` trait is properly implemented for all terminal types

### 2. Fix test setup
- Update tests to use existing methods rather than `new_with_seed`
- Ensure the test methods are available in the public API

### 3. Complete the build process
- Run final tests to ensure everything compiles and works properly 