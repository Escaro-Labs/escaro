## What this changes

<!-- One or two sentences. What does a user or contributor see differently? -->

Closes #

## How I tested it

<!-- Commands run, flows clicked through, browsers or networks used. -->

- [ ] `npm run build`
- [ ] `npm run test:smoke -- http://localhost:3000`
- [ ] Contract: `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (if `contracts/` changed)

## Screenshots

<!-- For any visible UI change: before and after, in light and dark themes. Delete if not applicable. -->

## Checklist

- [ ] Meets every acceptance criterion on the linked issue, or explains below which it does not
- [ ] Updated the smoke test if a covered flow changed
- [ ] Updated docs (`README.md`, `contracts/README.md`, `docs/architecture.md`) if behaviour changed
- [ ] No hard-coded product, chain or asset names (use `src/lib/platform.ts`)

## Notes for reviewers

<!-- Anything you are unsure about, trade-offs you made, or follow-ups. -->
