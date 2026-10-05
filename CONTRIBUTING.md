# Contributing to Parallax

Thanks for helping. This guide covers setup, the checks your pull request must pass, and how
issues are scoped.

## Setup

You need Node.js 22 or newer.

```bash
npm install
npm run dev        # http://localhost:3000
```

On Windows PowerShell, use `npm.cmd` in place of `npm` if script execution is restricted.

## Checks

Every pull request runs these in CI. Run them locally before you push:

```bash
npm run build                                 # typecheck + production build
npm run test:smoke -- http://localhost:3000   # with the dev server running
```

The smoke test drives a headless Chromium through every surface. It uses `CHROME_PATH` if set,
the stock Chrome install on Windows, and Playwright's own Chromium elsewhere
(`npx playwright-core install chromium` installs it). On failure it saves a screenshot to
`test-results/`.

If your change alters a flow the smoke test covers, update the test in the same pull request.

## Picking up an issue

- Comment on the issue with a short plan before you start, and wait to be assigned. One
  contributor is assigned per issue.
- Each issue lists acceptance criteria. Your pull request is reviewed against those, so call out
  anything you could not meet.
- Keep a pull request to one issue. Link it with `Closes #<number>`.

## Code style

- Match the surrounding code: TypeScript, function components, the existing design tokens in
  `src/styles/base.css`.
- Product, chain and asset names come from `src/lib/platform.ts`. Do not hard-code them in copy.
- Data shapes and pure helpers live in `src/lib/model.ts`. If you change `State`, bump its
  `version` and the `storageKey` so old browser data is discarded instead of misread.
- Every animated component must stay fully visible under `prefers-reduced-motion`.

## Commit messages

Use a short conventional prefix: `feat:`, `fix:`, `docs:`, `test:`, `chore:`, `refactor:`.
