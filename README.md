# Parallax

**Per-issue bounties for open-source work, escrowed and settled on Stellar.**

Maintainers put a fixed USDC bounty on a single issue. The money is locked in escrow on Stellar
when the issue is posted and released to the contributor the moment their pull request merges.
Every payout leaves a public receipt, so a contributor's record of shipped work travels with them.

Two views of the same work: maintainers see a backlog that gets done, contributors see money that
is already there.

> **Status: preview.** The full product flow runs in the browser with local data. The escrow
> contract, wallet connection and GitHub integration are on the [roadmap](#roadmap) and not
> connected yet. No real funds move.

## How it differs from a funding round

| | Funding rounds | Parallax |
| --- | --- | --- |
| Unit of funding | A pool for a time window | One issue |
| Price | Points converted to a share of the pool later | Fixed bounty, known before you apply |
| When money is committed | When the round is funded | When the issue is posted, in escrow |
| When you are paid | When the round closes | When your pull request merges |
| Record of work | Platform history | A public receipt per payout |

Parallax can sit alongside grant programs. A project in a funding round can still fund an urgent
issue directly.

## Run

Requires Node.js 22 or newer.

```bash
npm install
npm run dev     # http://localhost:3000
```

## Surfaces

**Public** — top nav, centred content.

| Route | |
| --- | --- |
| `/` | Landing |
| `/explore`, `/explore/repos`, `/explore/orgs` | Browse funded issues, repositories, organizations |
| `/issue/:id` | Issue detail, bounty, escrow state and proposals |

**Workspaces** — fixed left rail, scoped to one role or one repository.

| Route | |
| --- | --- |
| `/login` | Contributor sign-in |
| `/me` | Assignments and pull request submission |
| `/me/receipts` | Earned total and a receipt per paid bounty |
| `/me/settings` | Display name and Stellar payout address |
| `/maintainer/login` | **Separate** maintainer sign-in |
| `/maintainer` | Your repositories and their verification status |
| `/maintainer/submit` | Connect a repository |
| `/maintainer/repo/:id` | Dashboard: escrowed and paid-out totals, proposals waiting |
| `/maintainer/repo/:id/issues` | Post funded issues, assign, merge and release |

## The flow

1. **Connect.** A maintainer signs in at `/maintainer/login` and connects a repository. It is
   `Pending` until maintainer access is verified. **No dashboard opens before that.**
2. **Fund.** From the repository dashboard, the maintainer posts an issue with acceptance criteria
   and a bounty. Complexity suggests a starting price; the maintainer sets the final amount.
3. **Apply.** Contributors send a short plan. The maintainer assigns one; the others are declined
   automatically.
4. **Build.** The assignee submits their pull request URL.
5. **Merge and release.** The maintainer merges, the bounty is released to the contributor's
   Stellar address, and a receipt is written.

The maintainer gate is enforced on the route, not just in the UI: a direct URL to an unverified
repository's dashboard, or to one owned by a different maintainer, redirects away. Contributor and
maintainer are separate sessions; signing into one never opens the other.

## Project layout

| Path | |
| --- | --- |
| `src/lib/platform.ts` | Product name, chain, network and asset. All copy reads from here. |
| `src/lib/model.ts` | Types, pure helpers, the dashboard gate and seed data |
| `src/lib/store.tsx` | App state in one React context, persisted to `localStorage` |
| `src/App.tsx` | Routes, public shell and workspace shells |
| `src/pages/` | One file per surface |
| `src/components/ui.tsx` | Shared primitives: avatar, modal, chips, segmented control |
| `src/components/bits/` | Motion and WebGL effects ported from React Bits |
| `scripts/smoke-test.mjs` | End-to-end test of every flow at five viewport widths |

## Checks

```bash
npm run build                                 # typecheck + production build
npm run test:smoke -- http://localhost:3000   # with the dev server running
```

The smoke test covers the public explore surface, search, tabs and filters, contributor apply and
persistence, the separate maintainer sign-in, the connect-then-verify gate, dashboards scoped by
owner, assignment, posting a funded issue, payout address validation, merge-and-release writing a
receipt, theme persistence, the mobile drawer, and horizontal overflow across 15 routes at five
widths. CI runs the build and the smoke test on every pull request.

## Roadmap

1. **Escrow contract (Soroban).** `fund(issue, amount)`, `assign(issue, contributor)`,
   `release(issue)` and `refund(issue)`, holding USDC per issue. Deployed to Stellar testnet first.
2. **Receipts on-chain.** `release` emits an event with repository, issue, pull request and
   amount; the receipts page reads them back from the network.
3. **Wallets.** Freighter for maintainers funding issues; passkey smart wallets so contributors
   can get paid without a seed phrase. Payouts to accounts without a USDC trustline go out as
   claimable balances.
4. **GitHub app.** Real sign-in, repository ownership verification from GitHub permissions, and
   release triggered by the merge itself.

See [CONTRIBUTING.md](CONTRIBUTING.md) to help build any of these.

## Design

Dark by default, with a violet accent. One neutral ramp plus one accent drives both themes. Every
radius token is `0`, so controls and cards are square; only true dots stay circular. Inter for UI,
JetBrains Mono for identifiers. Motion runs on one easing curve via `motion`, and every animated
component falls back to fully visible under `prefers-reduced-motion`.

## Data boundaries

Everything in the preview is local to your browser. The seeded repositories are **real, existing
open-source projects** used as reference examples so the directory renders with genuine avatars;
their star counts, topics, issues and bounties are fixtures, and their presence here implies no
affiliation with, or participation in, Parallax. Sample contributors (`nadia.dev`, `kwame-o`,
`lucia-m`, `tobi.k`) are fictional.

## License

[MIT](LICENSE)
