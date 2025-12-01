# Module: loading (TUI Widget)

## Functional Requirements

- Display a simple loading message to the user in the TUI.
- Inform the user to wait for the backend/server to start.
- Use a Ratatui widget (e.g., `Paragraph`) for rendering.
- Center the message in the available area.
- Message should be customizable but defaults to something like: "Please wait, backend is starting..."

## Technical Requirements

- Uses the `ratatui` crate for TUI rendering.
- Uses the `tachyonfx` crate for animated borders.
- Should be implemented as a reusable widget (e.g., `LoadingWidget`).
- Border animation should be a glowing border

## Manual Testing Scenarios

- [ ] Start the TUI while backend is initializing. Confirm the loading message is visible and centered.
- [ ] Change the loading message and confirm the update is reflected in the UI.
- [ ] Resize the terminal and confirm the widget remains visible and centered.
- [ ] Observe the animated border and confirm it updates smoothly.
