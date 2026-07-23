# TAXLANE Demo Script

Use this when showing TAXLANE live to a first-time reviewer. The goal is to show
the product judgment: every public fiscal claim carries its source basis, scope,
denominator, and claim status before it asks for trust.

## Five-Minute Version

1. Open [README.md](../README.md) and state the one-line purpose:
   TAXLANE is a source-backed civic visibility system for income-tax receipts,
   funding lanes, benchmark gaps, and reform proposals.
2. Open [Honest Federal Tax Receipt](reading/honest-federal-tax-receipt.md).
   Show the core standard: current law, legal dedication, modeled allocation,
   civic illustration, and reform proposal must not be mixed.
3. Open [Budget-Area Funding Explainer](reading/budget-area-funding-explainer.md).
   Show the research thesis in public language: health cost, old-age base,
   defense strategy, family support, and revenue adequacy are different
   problems.
4. Open [Current Versus Benchmark Scoreboard](reading/current-versus-benchmark-scoreboard.md).
   Show the firewall between benchmark gaps, improper payments, fraud,
   recoverable savings, and public claims.
5. Open the
   [adaptive-rate current frontier](../context/waves/2026-07-18-adaptive-rate-performance-system/WAVE.md#current-frontier).
   Show that the active implementation surface is gated source lineage and
   readiness, not published rates.
6. Run:

   ```powershell
   cargo run -p taxlane-tools -- income-tax-outlay validate
   ```

   Explain that the validator checks derived records, public packet links,
   chart specs, manifests, and claim boundaries.

## Fifteen-Minute Version

Start with the five-minute path, then add:

1. Open [Payment Integrity: What The Public Evidence Shows](reading/payment-integrity-bounded-factual-examples.md).
   Show exact reported public numbers while explaining why they do not become
   fraud, waste, or recoverable-savings claims.
2. Open [Headline Number Selection Guide](reading/headline-number-selection-guide.md).
   Show why health, defense, and interest numbers must keep scope and
   denominator visible.
3. Open [docs/papers](papers/README.md) and start with
   `0+legible-federal-funding.pdf`.
   Explain that PDFs are convenience renders and markdown remains the source
   tier.
4. Open [CONTRIBUTING.md](../CONTRIBUTING.md).
   Show that outside use is welcome, but public claims must preserve source
   custody and label discipline.

## What To Emphasize

- The repo is already useful as a public research artifact and reviewable
  knowledge base.
- The strongest product contribution is not a single number. It is the claim
  discipline around public fiscal numbers.
- The adaptive-rate work is the implementation frontier: it names gates,
  blockers, and nulls so the model does not silently invent readiness.
- The validator is part of the pitch because it makes the evidence firewall
  reproducible.

## What To Avoid

- Do not present TAXLANE as personal tax advice.
- Do not say the receipt legally traces a taxpayer's exact dollars.
- Do not call benchmark gaps savings.
- Do not call improper payments fraud or waste without a source-backed finding.
- Do not imply public rates, solver outputs, transportation savings, or a
  balanced-budget plan are ready.
