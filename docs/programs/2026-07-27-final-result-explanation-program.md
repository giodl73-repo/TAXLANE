# Taxlane final-result explanation program

## Purpose

Turn Taxlane's completed fifteen-track internal analysis into a coherent,
auditable civic-education corpus for readers with very different levels of
fiscal knowledge. The future audience may be national. The current execution
scope is repository-only: create sources, generated previews, and review
evidence here, without deploying a site, distributing a release, contacting an
organization, or initiating an official process.

The program succeeds when a citizen can understand the conclusion, a policy
reader can inspect its reasoning, and a technical reviewer can reproduce its
boundaries from the same source spine.

## The story to explain

Taxlane asks what federal services cost, what evidence supports changing those
costs, and what revenue schedule follows when unsupported savings are excluded.
It examined fifteen tracks, admitted no FY2026 primary spending reduction,
kept PAY non-additive and NET endogenous, and retained an $813.727 billion
revenue target. Its preferred internal analytical schedule is
21/23/33/35/43/46/48, with explicitly labeled behavior and stress alternatives.

That result is important because it demonstrates disciplined decision-making,
not because it claims omniscience. Zero admission means the tested spending
candidates did not clear their evidence gates. It does not mean reform is
impossible. The rate schedule is an internal analytical recommendation, not
enacted law, personal tax advice, an official score, or formal proof of balance.

## Audience ladder

| Audience | Question | Primary form | Required depth |
|---|---|---|---|
| Citizen | What did Taxlane conclude, and why should I trust the process? | one-page brief, visual guide, FAQ | plain language, visible caveats |
| Educator / journalist | How can I explain the result without flattening it? | teaching guide, glossary, briefing deck | definitions, examples, claim labels |
| Policy reader | What happened in each track and why was it admitted or rejected? | fifteen-track atlas, methods guide, papers | sources, gates, alternatives |
| Technical auditor | Can I reproduce the result and detect unsupported claims? | evidence map, machine records, validation guide | paths, hashes, equations, tests |

Each surface must route downward to more evidence. No simplified surface may
become a separate factual authority.

## Program waves

### EXPL-A — Narrative and evidence foundation

Status: complete after round-two `.roles` acceptance (Pulse 481).

Create the master narrative, claim ledger, number ledger, terminology guide,
and visual grammar. Establish one canonical statement for the result and one
canonical caveat block used by every downstream form.

Exit gate: every headline number and conclusion maps to a validated artifact;
no external release mechanism exists.

### EXPL-B — Citizen and teaching guides

Status: complete after round-two `.roles` acceptance (Pulse 482).

Build a one-page result, a short citizen guide, a longer guide to the fifteen
tracks, a frequently asked questions document, a glossary, and an educator's
discussion guide. Explain budget accounting, why zero admission is meaningful,
and how the analytical rate differs from a statutory or personal rate.

Exit gate: non-specialist review can accurately restate the result and its
caveats without consulting the technical corpus.

### EXPL-C — Research-paper series

Status: complete after round-two `.roles` and publication-panel acceptance
(Pulse 483); canonical Markdown and repository-only PDFs exist.

Develop four source-of-truth Markdown papers:

1. *The Taxlane Result* — the full synthesis.
2. *Why Zero Is a Result* — evidence gates and rejected savings.
3. *Fifteen Tracks, One Accounting Spine* — lane-specific reasoning and shared
   interfaces.
4. *From Spending Evidence to an Adaptive Rate* — the revenue model,
   behavioral cases, uncertainty, and limits.

Use the existing panel-review and PDF rendering conventions, but do not treat a
rendered PDF as a separate source or distribute it outside the repo.

Exit gate: all four papers clear sourcing, comparability, skepticism, and plain-
language review; Markdown remains canonical.

### EXPL-D — Presentation system

Status: complete after round-two `.roles` acceptance (Pulse 484); three
canonical Markdown decks and fully local review previews exist.

Create three presentations from shared claims and visuals:

- a five-minute overview;
- a twenty-minute civic briefing;
- a forty-five-minute technical walkthrough.

Each deck includes speaker notes, a caveat slide, a fifteen-track disposition
view, the PAY–NET–REV identity, the central/contingency/stress rate distinction,
and a final “what would change the result?” slide. Keep deck sources in the repo;
generated previews are local artifacts only.

Exit gate: every slide claim is traceable and the three decks do not drift in
numbers, labels, or scope.

### EXPL-E — Local HTML experience

Status: complete after round-two `.roles` acceptance (Pulse 485); the
repository-contained six-page static review surface passes local safety gates.

Build a static, accessible, responsive HTML explanation with progressive
disclosure:

- result overview;
- fifteen-track explorer;
- rate and uncertainty explainer;
- evidence and methods routes;
- glossary and FAQ;
- download links only to repo-contained source artifacts.

The HTML is a local preview and review surface. No hosting configuration,
analytics, mailing list, tracking pixel, deployment credential, or external
distribution action belongs in this wave.

Exit gate: local build, accessibility, link, responsive-layout, and claim-sync
checks pass; deployment remains disabled.

### EXPL-F — Integrated review and repository readiness

Status: complete after round-two `.roles` acceptance (Pulse 486). All 21
canonical deliverables are repository-ready; external release remains blocked.

Run cross-format numerical consistency, eight-role review, accessibility,
reading-level, source-link, reproduction, adversarial-claim, and archival
checks. Produce a repository-contained briefing bundle and a release-readiness
decision record.

Exit gate: the corpus is internally coherent and review-ready. External release
remains blocked unless the owner later provides a new, explicit authorization.

## Canonical content portfolio

| Family | Planned artifacts | Source of truth |
|---|---:|---|
| Result briefs and guides | 6 | Markdown |
| Research papers | 4 | Markdown under `research/publications/` |
| Presentation variants | 3 | shared Markdown/data sources |
| Local HTML experience | 1 multi-page static build | templates plus validated data |
| Evidence and methods maps | 3 | Markdown plus machine-readable indexes |
| Review and consistency reports | 4 | Markdown and generated check output |

The initial portfolio is 21 deliverables. Variants generated from one canonical
source do not count as additional independent publications.

## Shared content modules

All forms reuse these modules rather than rewriting facts independently:

1. canonical result statement;
2. fifteen-track terminal disposition table;
3. PAY–NET–REV accounting identity;
4. preferred, behavior-robust, and stress schedule comparison;
5. admitted-versus-rejected evidence explanation;
6. “what would change the result?” reopening triggers;
7. limitations and non-claims;
8. source and reproduction routes.

## Review gates

- **Numerical identity:** $813.727 billion, all schedules, candidate counts,
  admitted amounts, and test counts agree across formats.
- **Claim class:** every statement is labeled as sourced fact, model result,
  interpretation, recommendation, uncertainty, or blocked claim.
- **Traceability:** important claims link to a reader artifact and a machine
  artifact.
- **Accessibility:** semantic structure, keyboard navigation, contrast,
  alternative text, table fallbacks, and reduced-motion behavior are checked.
- **Comprehension:** citizen surfaces define terms before using them and avoid
  making bracket rates sound like one person's effective tax rate.
- **Adversarial reading:** reviewers test partisan reframing, false precision,
  savings inflation, improper-payment conflation, and official-score mimicry.
- **Format parity:** Markdown, PDF preview, slide preview, and local HTML report
  the same result and caveat boundary.
- **Release control:** repository readiness never implies authorization to
  deploy, announce, transmit, or solicit endorsement.

## Operating order

EXPL-A is the only starting wave. EXPL-B and EXPL-C may run after its narrative
and number ledgers close. EXPL-D starts after the short guide and synthesis-paper
outlines stabilize. EXPL-E starts after the shared content modules and visual
grammar stabilize. EXPL-F runs only after all prior sources exist.

The first implementation bundle should therefore contain:

1. canonical result statement;
2. claim and number ledgers;
3. complete artifact inventory;
4. citizen-guide outline;
5. synthesis-paper outline;
6. presentation narrative spine;
7. local-HTML information architecture;
8. review matrix.

## Explicit boundary

This program authorizes documentation, source material, generated previews,
tests, and reviews inside the Taxlane repository. It does not authorize a public
release, deployment, domain change, social post, email, press contact,
stakeholder outreach, government contact, official score request, legislative
submission, or representation that any person or institution endorsed Taxlane.
