import { Link } from 'react-router-dom';
import { ArrowRight, GitPullRequest, Send, Wallet } from 'lucide-react';
import { formatCount, formatMoney, repoName, type Issue, type Repo } from '../lib/model';
import { PLATFORM } from '../lib/platform';
import { Avatar, Chip } from '../components/ui';
import { AnimatedContent, BounceCards, CardSwap, InfiniteSpiral, ProfileCard } from '../components/bits';

/** Stacked issue cards that cycle, so the backlog reads as live. */
export function LiveStackSection({ issues, repos }: { issues: Issue[]; repos: Repo[] }) {
  const cards = issues.slice(0, 4).map(issue => {
    const repo = repos.find(r => r.id === issue.repoId);
    return {
      id: issue.id,
      node: (
        <Link className="stack-card" to={`/issue/${issue.id}`}>
          <div className="row">
            <Chip className="solid num">${formatMoney(issue.bounty)}</Chip>
            <Chip>{issue.complexity}</Chip>
            <span className="spacer" />
            <span className="mono dim" style={{ fontSize: 'var(--t2)' }}>#{issue.id}</span>
          </div>
          <h3 className="stack-title">{issue.title}</h3>
          <p className="stack-desc">{issue.description}</p>
          <div className="row stack-foot">
            {repo && <Avatar name={repo.org} org={repo.org} square />}
            <span className="row-sub">{repo ? repoName(repo) : ''}</span>
            <span className="spacer" />
            <ArrowRight size={13} className="dim" />
          </div>
        </Link>
      ),
    };
  });

  return (
    <section className="band">
      <div className="band-inner split-wide">
        <AnimatedContent>
          <div>
            <h2 className="sec-h">The backlog, live</h2>
            <p className="muted band-copy">
              Issues cycle through the queue as maintainers fund them. Every card carries its bounty
              and acceptance criteria, and the money is already in escrow before anyone applies.
            </p>
            <Link className="btn" to="/explore">Open the board<ArrowRight size={14} /></Link>
          </div>
        </AnimatedContent>
        <AnimatedContent delay={0.1}>
          <CardSwap cards={cards} />
        </AnimatedContent>
      </div>
    </section>
  );
}

/** Three lifecycle cards that fan out on scroll. */
export function FanSection() {
  const cards = [
    { icon: Send, t: 'Apply', d: 'Send a short plan. No speculative pull requests.', tone: 'a' },
    { icon: GitPullRequest, t: 'Build', d: 'One assignee per issue, reviewed against the criteria it was funded on.', tone: 'b' },
    { icon: Wallet, t: 'Get paid', d: `Merge releases the bounty in ${PLATFORM.asset} straight to your ${PLATFORM.chain} address.`, tone: 'c' },
  ];
  return (
    <section className="band alt">
      <div className="band-inner">
        <AnimatedContent>
          <div className="sec centered-sec">
            <div>
              <h2>Three moves, start to paid</h2>
              <p>No rounds to wait for, no pool to split, no negotiation over price.</p>
            </div>
          </div>
        </AnimatedContent>
        <BounceCards>
          {cards.map(c => (
            <div className={`fan-card tone-${c.tone}`} key={c.t}>
              <span className="fan-icon"><c.icon size={18} /></span>
              <h3>{c.t}</h3>
              <p>{c.d}</p>
            </div>
          ))}
        </BounceCards>
      </div>
    </section>
  );
}

/** Repositories arranged on a rotating spiral. */
export function SpiralSection({ repos }: { repos: Repo[] }) {
  const items = repos.map(r => (
    <Link className="spiral-chip" to={`/explore?q=${encodeURIComponent(r.name)}`} key={r.id} title={repoName(r)}>
      <Avatar name={r.org} org={r.org} square />
      <span className="col" style={{ gap: 0, minWidth: 0 }}>
        <span className="row-title">{r.name}</span>
        <span className="row-sub">{formatCount(r.stars)} stars</span>
      </span>
    </Link>
  ));

  return (
    <section className="band">
      <div className="band-inner split-wide">
        <AnimatedContent delay={0.05}>
          <InfiniteSpiral items={items} />
        </AnimatedContent>
        <AnimatedContent>
          <div>
            <h2 className="sec-h">One ecosystem, every layer</h2>
            <p className="muted band-copy">
              Contracts, tooling, wallets and docs — any repository in the {PLATFORM.chain} ecosystem
              can fund a single issue the day it needs doing, without waiting for a grant round.
            </p>
            <Link className="btn" to="/explore/repos">See repositories<ArrowRight size={14} /></Link>
          </div>
        </AnimatedContent>
      </div>
    </section>
  );
}

/** Sample contributors, as tilting holographic cards. */
export function TeamSection({ repos }: { repos: Repo[] }) {
  const team = [
    { name: 'Nadia Osei', role: 'Soroban tooling', handle: '@nadia.dev', org: repos[0]?.org, stat: '14 receipts' },
    { name: 'Kwame Owusu', role: 'CLI and RPC', handle: '@kwame-o', org: repos[1]?.org, stat: '9 receipts' },
    { name: 'Lucía Marín', role: 'Wallets and signing', handle: '@lucia-m', org: repos[3]?.org, stat: '11 receipts' },
    { name: 'Tobi Kalu', role: 'SDKs and types', handle: '@tobi.k', org: repos[2]?.org, stat: '7 receipts' },
  ];

  return (
    <section className="band alt">
      <div className="band-inner">
        <AnimatedContent>
          <div className="sec centered-sec">
            <div>
              <h2>Work that follows you</h2>
              <p>Every paid bounty leaves a receipt, so a contributor's record travels with them.</p>
            </div>
          </div>
        </AnimatedContent>
        <div className="grid c4 team-grid">
          {team.map((m, i) => (
            <AnimatedContent key={m.handle} delay={i * 0.07}>
              <ProfileCard
                name={m.name}
                role={m.role}
                handle={m.handle}
                stat={m.stat}
                avatar={<Avatar name={m.name} org={m.org} square />}
              />
            </AnimatedContent>
          ))}
        </div>
        <p className="hint team-note">
          Sample contributors for this preview — not real people.
        </p>
      </div>
    </section>
  );
}
