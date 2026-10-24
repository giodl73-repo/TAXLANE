const repo = name => `https://github.com/giodl73-repo/${name}`;
const owner = (name, scope, demo = null) => ({name, scope, source:repo(name), demo});
const site = name => `https://giodl73-repo.github.io/${name}/`;
export const laneEvidence = {
  'national-defense': {owners:[owner('BASTION','synthetic support-remedy comparison',site('BASTION'))], note:'The fictional non-operational workbench preserves nine support facets and safety/evidence holds. Readiness is not established by spending or inventory; no remedy is selected.'},
  'international-affairs': {owners:[owner('ENVOY','assistance realization and safeguards',site('ENVOY'))], note:'The workbench uses synthetic aggregate portfolios. Obligations and disbursements do not by themselves establish delivered outcomes.'},
  'science-space-technology': {owners:[], note:'Domain owner unassigned. Fiscal records do not establish science or space program performance.'},
  'environment-energy-natural-resources': {owners:[owner('PYLON','transmission graph arithmetic',site('PYLON')),owner('BASIN','frozen USGS daily streamflow and separate fictional conveyance ratings',site('BASIN')),owner('DRAIN','frozen EPA CWNS reported capital-needs composition and separate fictional sewer graph',site('DRAIN'))], note:'These owners cover narrower domains, not the whole environment/energy lane. BASIN compares frozen Lees Ferry daily discharge with coverage and qualifiers; its conveyance graph remains fictional. Observed flow does not establish allocable supply, firm yield or safe access. DRAIN retains January 2022-dollar reported capital needs over a 20-year horizon, not annual spending or savings. Synthetic topology does not establish physical flow, water rights/quality, treatment, permits or receiving-water impact.'},
  'agriculture': {owners:[], note:'Domain owner unassigned. Agriculture program outcomes remain outside the budget calculation.'},
  'commerce-housing-credit': {owners:[], note:'Domain owner unassigned. This negative net accounting row is held fixed; it is not a service-performance or housing outcome.'},
  'transportation': {owners:[owner('ROUTE','synthetic freight and whole-trip scenarios',site('ROUTE')),owner('GAUGE','historical rail frequencies and hypothetical portfolios',site('GAUGE')),owner('TARMAC','frozen BTS arrival outcomes and separate fictional regional connectivity',site('TARMAC')),owner('HARBOR','historical gateway-score distribution replay',site('HARBOR'))], note:'Modes retain their own assumptions and evidence. TARMAC compares frozen January–August reporting-carrier arrivals, with changing coverage; its fictional graph is separate. Neither aggregate outcomes nor airport topology establish passenger itineraries or actual access. HARBOR preserves unassessed dimensions. Budget changes do not predict delivered service.'},
  'community-regional-development': {owners:[owner('ZONES','civic boundaries only',site('ZONES'))], note:'The broader regional-program domain is unassigned. ZONES does not establish housing, disaster or regional-development outcomes.'},
  'education-training-employment-social-services': {owners:[owner('SLATE','frozen NCES enrollment/staffing comparisons and separate synthetic school pathways',site('SLATE'))], note:'Frozen school-year counts and pupil/teacher ratios do not establish class size, staffed seats, eligible progression or learning. Source missingness stays visible; the fictional topology is separate. Employment and general social-service semantics remain unassigned.'},
  'health': {owners:[owner('SHIELD','frozen CMS hospital footprint and county-rurality source exploration',site('SHIELD'))], note:'Source registration and county classification do not establish patient access, clinical outcomes, eligibility or quality. The rurality join retains unmatched facilities.'},
  'medicare': {owners:[owner('SHIELD','hospital source exploration only',site('SHIELD'))], note:'TAXLANE retains financing distinctions. SHIELD does not own Medicare eligibility or payer policy; registered facilities are not staffed care.'},
  'income-security': {owners:[owner('LIFELINE','aggregate household resources, access and benefit continuity',site('LIFELINE'))], note:'The workbench uses synthetic aggregate paths. Lower participation or benefits cannot be relabeled as savings. No individual eligibility decision is made.'},
  'social-security': {owners:[owner('ANCHOR','aggregate retirement and disability service access and adequacy',site('ANCHOR'))], note:'The workbench compares synthetic aggregate service and adequacy examples. Service access, benefit adequacy and dedicated solvency remain separate. Research candidates require fiscal admission.'},
  'veterans': {owners:[owner('COVENANT','aggregate service handoffs and continuity',site('COVENANT'))], note:'The workbench uses synthetic aggregate handoffs. Authorization does not establish completed care or benefits. No individual claims decision is made.'},
  'justice-general-government': {owners:[owner('TRIBUNAL','frozen AOUSC caseflow reconciliation and separate synthetic rights floors',site('TRIBUNAL'))], note:'The workbench inspects frozen AOUSC national caseflow for the year ending March 31, 2025, one segment at a time. Cases and criminal defendants stay separate; signed reconciliation residuals are unexplained by the source table. Rights quality, delay, costs and savings are unobserved. Its synthetic aggregate caseflow comparison remains separate. General-government performance remains unassigned. Throughput does not establish rights-compliant outcomes.'},
  'net-interest': {owners:[owner('TAXLANE','source-custodied financing accounting')], note:'Held fixed: financing cost, not a service outcome or assumed interest saving.'},
  'undistributed-offsetting-receipts': {owners:[owner('TAXLANE','source-custodied offset accounting')], note:'Held fixed: offsets reduce net outlays. They are not automatically efficiency, fraud reduction or program savings.'},
};

export function evidencePanel(lane) {
  const record=laneEvidence[lane.id];
  if(!record) throw new Error(`Missing evidence metadata for ${lane.id}`);
  const details=document.createElement('details');details.className='lane-evidence';details.id=`evidence-${lane.id}`;
  const summary=document.createElement('summary');summary.textContent=`Evidence & service tools: ${lane.label}`;
  const basis=document.createElement('p');basis.textContent=`Budget basis: FY2025 · OMB FY2027 source vintage · ${lane.sources.join(', ')}. Changes here are hypothetical accounting inputs.`;
  const scope=document.createElement('p');scope.textContent=record.note;
  details.append(summary,basis,scope);
  for(const provider of record.owners) {
    const p=document.createElement('p');const source=document.createElement('a');source.href=provider.source;source.textContent=`${provider.name} research & evidence`;
    p.append(source,` — ${provider.scope}. `);
    if(provider.demo) {const demo=document.createElement('a');demo.href=provider.demo;demo.textContent=`Open ${provider.name} workbench`;p.append(demo);}
    details.append(p);
  }
  const status=document.createElement('p');status.className='evidence-status';status.textContent='Service evidence and fiscal admission are separate. These tools do not admit savings or change TAXLANE’s research tax-rate results.';
  const jurisdiction=document.createElement('a');jurisdiction.href=`${repo('TAXLANE')}/blob/main/docs/jurisdiction-map.md`;jurisdiction.textContent='Full jurisdiction and handoff contract';
  details.append(status,jurisdiction);return details;
}
