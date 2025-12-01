# Module: loading (TUI Widget)

## Functional Requirements

- Display a simple loading message to the user in the TUI.
- Inform the user to wait for the backend/server to start.
- Use a Ratatui widget (e.g., `Paragraph`) for rendering.
- Center the message in the available area.
- Display an animated border using the `tachyonfx` crate.

## Technical Requirements

- Uses the `ratatui` crate for TUI rendering.
- Uses the `tachyonfx` crate for animated borders.
- Should be implemented as a reusable widget (e.g., `LoadingWidget`).
- Message should be customizable but defaults to something like: "Please wait, backend is starting..."
- The border animation should use `tachyonfx::AnimatedBorder` and update its frame on each render or tick.
- The animation can use the default style or be customized (e.g., color, title).

## Example Animation Usage

```rust
use tachyonfx::AnimatedBorder;

// In your widget's render method:
let border = AnimatedBorder::default()
	.frame(animation_frame)
	.title("Loading")
	.style(Style::default().fg(Color::Cyan));
border.render(area, buf);
```
- `animation_frame` should be incremented on each tick to animate the border.
- The message should be rendered inside the border, centered.

## Auto Testing Scenarios

- Render the widget and verify the message is displayed and centered.
- Change the message and verify the new message is rendered.
- Widget does not panic or crash if area is small.
- Border animation updates as the frame changes.

## Manual Testing Scenarios

- [ ] Start the TUI while backend is initializing. Confirm the loading message is visible and centered.
- [ ] Change the loading message and confirm the update is reflected in the UI.
- [ ] Resize the terminal and confirm the widget remains visible and centered.
- [ ] Observe the animated border and confirm it updates smoothly.
