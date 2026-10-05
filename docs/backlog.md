# Backlog

Scoped tasks ready to open as GitHub issues. Each has acceptance criteria and a complexity level.
Open them with the **Scoped task** issue template and add the matching labels.

---

## 1. Connect Freighter and show the maintainer's Stellar address

Labels: `task`, `web`, `complexity: medium`

**Context.** Maintainers will fund bounties from their own Stellar account, so the maintainer area
needs a wallet connection. [Freighter](https://www.freighter.app/) is the common browser wallet
for Stellar and ships `@stellar/freighter-api`.

**Acceptance criteria**
- [ ] A "Connect wallet" button in the maintainer workspace rail footer connects Freighter.
- [ ] Once connected, the rail shows the short address (`shortAddress` in `src/lib/model.ts`) and the network Freighter is on.
- [ ] If Freighter is on a different network from `PLATFORM.network` (`src/lib/platform.ts`), show a clear warning and do not treat the wallet as ready.
- [ ] If Freighter is not installed, the button links to the install page instead of failing silently.
- [ ] The connection survives a reload, and a "Disconnect" action clears it.
- [ ] `npm run build` and the smoke test pass. Stub the wallet API in the smoke test.

**Pointers.** `src/App.tsx` (`MaintainerShell`, `Rail` footer) · https://docs.freighter.app/

---

## 2. Deploy the escrow contract to testnet and document the contract id

Labels: `task`, `contract`, `complexity: trivial`

**Context.** The escrow contract in `contracts/escrow` is tested but never deployed. The app needs
a known testnet contract id before it can call it.

**Acceptance criteria**
- [ ] Build with `stellar contract build` and deploy to testnet, using testnet USDC's Stellar Asset Contract as the `token` constructor argument.
- [ ] Verify the USDC issuer against Circle's published testnet address and cite the source in the PR.
- [ ] Add the contract id to `src/lib/platform.ts` as `escrowContractId`.
- [ ] Update "Deploy to testnet" in `contracts/README.md` with the exact commands and the resulting id.
- [ ] From the CLI, fund a small bounty, assign, and release. Link the three transactions in the PR.

**Pointers.** `contracts/README.md` · https://developers.stellar.org/docs/tools/cli

---

## 3. Fund a bounty on-chain when a maintainer posts an issue

Labels: `task`, `web`, `contract`, `complexity: high` · Depends on 1 and 2

**Context.** "Post and fund issue" only writes local state. With a deployed contract and Freighter
connected, posting should call the contract's `fund`.

**Acceptance criteria**
- [ ] Posting builds a `fund(maintainer, repo, issue, amount)` invocation with `@stellar/stellar-sdk`, simulates it, asks Freighter to sign, and submits it to Soroban RPC.
- [ ] Amounts are converted to USDC's 7 decimal places correctly, with a unit test.
- [ ] Pending, success and failure states are shown. On failure nothing is added to local state, and contract errors (e.g. `AlreadyFunded`) are shown in plain words.
- [ ] On success, the issue stores the bounty id and transaction hash, and the issue page links the transaction on a block explorer.
- [ ] With no wallet connected, posting keeps today's local-only behaviour and says so.
- [ ] The smoke test still passes without a real wallet.

**Pointers.** `src/pages/maintainer.tsx` (`PostIssueForm`) · `contracts/escrow/src/lib.rs` (`fund`)

---

## 4. Read receipts from contract events on the receipts page

Labels: `task`, `web`, `contract`, `complexity: high` · Depends on 2

**Context.** `/me/receipts` reads local storage. The contract emits a `receipt` event on every
release, with the contributor's address as a topic, so the page can show real public history.

**Acceptance criteria**
- [ ] With a payout address set, `/me/receipts` fetches `receipt` events for it from Soroban RPC (`getEvents`, filtered by contract id and topics).
- [ ] Each row shows repository, issue, pull request link, amount, date, and a block explorer link.
- [ ] Local preview receipts still show, labelled "preview", separate from on-chain ones.
- [ ] Loading, empty and RPC-error states are handled; an RPC failure never blanks the page.
- [ ] Event decoding lives in a small tested module, not inline in the component.

**Pointers.** `src/pages/contributor.tsx` (`ContributorReceipts`) · https://developers.stellar.org/docs/data/apis/rpc/api-reference/methods/getEvents

---

## 5. Let maintainers top up an open bounty

Labels: `task`, `contract`, `complexity: medium`

**Context.** A maintainer should be able to raise a bounty without refunding and re-funding,
which would lose the assignee.

**Acceptance criteria**
- [ ] Add `top_up(id, amount)`: maintainer-only, positive amount, bounty must be `Open`.
- [ ] Tokens move into escrow and `Bounty.amount` increases; the assignee is unchanged.
- [ ] Emit a `topped_up` event with the id, added amount and new total.
- [ ] Tests: top-up then release pays the new total; top-up on a paid bounty fails with `NotOpen`; a non-positive amount fails with `InvalidAmount`.
- [ ] Document it in `contracts/README.md`.
- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings` and `cargo test` pass.

**Pointers.** `contracts/escrow/src/lib.rs` (follow `assign` / `refund`) · `contracts/escrow/src/test.rs` (`setup()`)

---

## 6. Explore ignores `?q=` when you are already on `/explore`

Labels: `bug`, `web`, `complexity: trivial`

**What happens.** The search box reads `?q=` only on mount. Navigating to `/explore?q=…` from
within `/explore` changes the URL but not the search or results.

**Acceptance criteria**
- [ ] The search box and results follow `?q=` whenever it changes.
- [ ] Typing keeps the URL in sync using `replace`, not a history entry per keystroke.
- [ ] A smoke-test step reproduces the bug and passes with the fix.

**Pointers.** `src/pages/explore.tsx`: `useState(params.get('q') ?? '')`

---

## 7. Enforce the three-assignment limit for contributors

Labels: `task`, `web`, `complexity: medium`

**Context.** The workspace says "N of 3 concurrent slots in use", but nothing enforces it.

**Acceptance criteria**
- [ ] A contributor with 3 issues in `Assigned` or `PR submitted` cannot be assigned another; the maintainer's Assign button is disabled with a short reason.
- [ ] On the issue page, a contributor at the limit sees why they cannot apply.
- [ ] The limit is one constant in `src/lib/model.ts`, and the "of 3" copy reads from it.
- [ ] The smoke test covers the limit.

**Pointers.** `src/pages/contributor.tsx` · `src/pages/maintainer.tsx` (`assign`) · `src/pages/issue.tsx` (`canApply`)

---

## 8. Keyboard navigation for the segmented control

Labels: `task`, `web`, `accessibility`, `complexity: trivial`

**Context.** The Explore switcher uses `role="tablist"` / `role="tab"` without the matching
keyboard pattern.

**Acceptance criteria**
- [ ] Left/Right arrows move between tabs and wrap; Home/End jump to the ends.
- [ ] Only the selected tab is in the Tab order.
- [ ] Focus follows selection, with a visible focus ring in both themes.
- [ ] The fix is in the shared `Segmented` component.

**Pointers.** `src/components/ui.tsx` (`Segmented`) · https://www.w3.org/WAI/ARIA/apg/patterns/tabs/

---

## 9. Remove unused font packages and dead components

Labels: `task`, `web`, `complexity: trivial`

**Context.** `@fontsource-variable/plus-jakarta-sans` and `@fontsource/ibm-plex-mono` are never
imported. `DotGrid`, `ShinyText`, `SpotlightCard`, `TiltedCard` and `Marquee` are exported but
unused.

**Acceptance criteria**
- [ ] Remove both packages with `npm uninstall`, so npm regenerates the lockfile.
- [ ] Remove the unused components and any CSS only they use.
- [ ] A search confirms nothing references the removed names.
- [ ] Build and smoke test pass; note bundle size before and after in the PR.

**Pointers.** `package.json` · `src/components/bits/motion.tsx` · `src/styles/bits.css`
