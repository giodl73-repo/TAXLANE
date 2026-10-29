import fs from 'node:fs/promises';
import {test,expect} from '@playwright/test';

test('each budget row exposes scoped evidence and existing satellite tools without changing accounting',async({page})=>{
  await page.goto('/TAXLANE/#evidence-income-security');
  await expect(page.locator('#gap')).toHaveText('$1,774.684B');
  await expect(page.locator('#evidence-income-security')).toHaveAttribute('open','');
  await expect(page.locator('.lane-evidence')).toHaveCount(17);
  const transport=page.locator('#evidence-transportation');
  await transport.locator('summary').focus();await page.keyboard.press('Enter');
  await expect(transport).toHaveAttribute('open','');
  await expect(transport.getByRole('link',{name:'Open ROUTE workbench'})).toHaveAttribute('href','https://giodl73-repo.github.io/ROUTE/');
  await expect(transport.getByRole('link',{name:'Open GAUGE workbench'})).toHaveAttribute('href','https://giodl73-repo.github.io/GAUGE/');
  await expect(transport).toContainText('FY2025');
  await expect(transport).toContainText('hypothetical accounting inputs');
  await expect(transport).toContainText('January–August reporting-carrier arrivals');
  await expect(transport).toContainText('fictional graph is separate');
  await expect(transport).toContainText('twelve source-needed frequency inputs');
  await expect(transport).toContainText('without original timetable custody');
  await expect(transport).toContainText('Frequency does not establish measured span, operating days');
  await expect(transport).toContainText('frozen 2023 Los Angeles loaded and total TEU');
  await expect(transport).toContainText('publisher table/news discrepancy');
  await expect(transport).toContainText('Throughput does not establish capacity, dwell or navigation clearance');
  const income=page.locator('#evidence-income-security');
  await expect(income.getByRole('link',{name:'LIFELINE research & evidence'})).toHaveAttribute('href','https://github.com/giodl73-repo/LIFELINE');
  await expect(income).toContainText('cannot be relabeled as savings');
  const retirement=page.locator('#evidence-social-security');await retirement.locator('summary').click();
  await expect(retirement.getByRole('link',{name:'Open ANCHOR workbench'})).toHaveAttribute('href','https://giodl73-repo.github.io/ANCHOR/');
  await expect(retirement).toContainText('synthetic aggregate');
  await expect(retirement).toContainText('dedicated solvency remain separate');
  for(const [lane,repo] of [['international-affairs','ENVOY'],['veterans','COVENANT'],['justice-general-government','TRIBUNAL']]) {
    const panel=page.locator(`#evidence-${lane}`);await panel.locator('summary').click();
    await expect(panel.getByRole('link',{name:`Open ${repo} workbench`})).toHaveAttribute('href',`https://giodl73-repo.github.io/${repo}/`);
    await expect(panel).toContainText('synthetic aggregate');
  }
  for(const [lane,repos] of [['national-defense',['BASTION']],['environment-energy-natural-resources',['PYLON','BASIN','DRAIN']],['transportation',['TARMAC','HARBOR']],['community-regional-development',['ZONES']],['education-training-employment-social-services',['SLATE']],['health',['SHIELD']],['medicare',['SHIELD']],['income-security',['LIFELINE']]]) {
    const panel=page.locator(`#evidence-${lane}`);await panel.evaluate(element=>element.open=true);
    for(const repo of repos) await expect(panel.getByRole('link',{name:`Open ${repo} workbench`})).toHaveAttribute('href',`https://giodl73-repo.github.io/${repo}/`);
  }
  await expect(page.locator('#evidence-justice-general-government')).toContainText('frozen AOUSC national caseflow');
  await expect(page.locator('#evidence-justice-general-government')).toContainText('Cases and criminal defendants stay separate');
  await expect(page.locator('#evidence-income-security')).toContainText('frozen FY2024/FY2025 SNAP national monthly averages');
  await expect(page.locator('#evidence-income-security')).toContainText('Participation is not take-up');
  await expect(page.locator('#evidence-social-security')).toContainText('frozen SSA 2025 Supplement highlights');
  await expect(page.locator('#evidence-social-security')).toContainText('separate from annual 2024 payments');
  await expect(page.locator('#evidence-veterans')).toContainText('frozen FY2025 VBA program obligations');
  await expect(page.locator('#evidence-veterans')).toContainText('not cash outlays or total VA spending');
  await expect(page.locator('#evidence-national-defense')).toContainText('frozen 2025 NATO expenditure estimates');
  await expect(page.locator('#evidence-national-defense')).toContainText('different denominators');
  await expect(page.locator('#evidence-national-defense')).toContainText('not readiness, a target or savings');
  await expect(page.locator('#evidence-international-affairs')).toContainText('frozen FY2024 regional obligations and disbursements');
  await expect(page.locator('#evidence-international-affairs')).toContainText('not unspent balances');
  await expect(page.locator('#evidence-international-affairs')).toContainText('one-dollar source difference');
  await expect(page.locator('#evidence-health')).toContainText('status rows and designation IDs distinct');
  await expect(page.locator('#evidence-health')).toContainText('June 30 quarterly report is separately dated');
  await expect(page.locator('#evidence-medicare')).toContainText('A shortage registry is not Medicare eligibility');
  const science=page.locator('#evidence-science-space-technology');await science.locator('summary').click();
  await expect(page.locator('#evidence-education-training-employment-social-services')).toContainText('Frozen school-year counts');
  await expect(page.locator('#evidence-education-training-employment-social-services')).toContainText('fictional topology is separate');
  await expect(page.locator('#evidence-environment-energy-natural-resources')).toContainText('frozen Lees Ferry daily discharge');
  await expect(page.locator('#evidence-environment-energy-natural-resources')).toContainText('conveyance graph remains fictional');
  await expect(page.locator('#evidence-environment-energy-natural-resources')).toContainText('January 2022-dollar reported capital needs');
  await expect(page.locator('#evidence-environment-energy-natural-resources')).toContainText('not annual spending or savings');
  await expect(science).toContainText('Domain owner unassigned');
  await page.locator('#evidence-net-interest summary').click();
  await expect(page.locator('#evidence-net-interest')).toContainText('Held fixed');
  await expect(page.locator('#gap')).toHaveText('$1,774.684B');
  await page.setViewportSize({width:390,height:844});
  expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBeTruthy();
});

test('real Rust accounting changes a budget and shares a reproducible scenario',async({page})=>{
  const errors=[];page.on('pageerror',error=>errors.push(error.message));
  const wasm=page.waitForResponse(response=>response.url().endsWith('.wasm'));
  await page.goto('/TAXLANE/');expect((await wasm).status()).toBe(200);
  await expect(page.locator('#gap')).toHaveText('$1,774.684B');
  await expect(page.locator('.lane')).toHaveCount(14);
  await expect(page.locator('#lane-net-interest')).toHaveCount(0);
  await page.locator('#lane-health').focus();
  await page.locator('#lane-health').press('ArrowLeft');
  await expect(page.locator('#delta')).toContainText('less financing');
  await page.locator('#lane-health').evaluate(input=>{input.value='-10';input.dispatchEvent(new Event('input',{bubbles:true}));});
  await expect(page.locator('#gap')).toHaveText('$1,676.833B');
  await expect(page.locator('#changes')).toContainText('Health');
  await page.getByRole('button',{name:'Copy scenario link'}).click();
  const shared=page.url();expect(shared).toContain('scenario=');
  await page.reload();await expect(page.locator('#gap')).toHaveText('$1,676.833B');
  const download=page.waitForEvent('download');
  await page.getByRole('button',{name:'Download scenario'}).click();
  expect((await download).suggestedFilename()).toBe('taxlane-scenario.json');
  await page.getByRole('button',{name:'Reset to baseline'}).click();
  await expect(page.locator('#gap')).toHaveText('$1,774.684B');
  expect(errors).toEqual([]);
});

test('mobile controls fit and malformed shared input safely returns to baseline',async({page})=>{
  await page.setViewportSize({width:390,height:844});
  await page.goto('/TAXLANE/?scenario=invalid');
  await expect(page.locator('#gap')).toHaveText('$1,774.684B');
  await expect(page.locator('#status')).toContainText('Invalid shared scenario');
  expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBeTruthy();
});

test('failed baseline download offers a readable research route',async({page})=>{
  await page.route('**/baseline.json',route=>route.fulfill({status:503,body:'unavailable'}));
  await page.goto('/TAXLANE/');
  await expect(page.locator('#status')).toContainText('Reload to retry');
  await expect(page.getByRole('link',{name:'Research & sources'})).toBeVisible();
  await expect(page.locator('#reset')).toBeDisabled();
});

test('transit cost equivalents preserve suballocation, source, sharing and cuts',async({page})=>{
 await page.goto('/TAXLANE/');await expect(page.locator('#gap')).toHaveText('$1,774.684B');
 await page.locator('#transit-example').click();await expect(page.locator('#transit-detail')).toContainText('$160.84/hour');
 const expected=await page.locator('#transit-hours').textContent();expect(expected).not.toBe('0 vehicle revenue hours');
 const gap=await page.locator('#gap').textContent();await page.locator('#transit-share').fill('50');await expect(page.locator('#transit-share-value')).toHaveText('50%');await expect(page.locator('#transit-hours')).not.toHaveText(expected);await expect(page.locator('#gap')).toHaveText(gap);
 await page.locator('#share').click();const hours=await page.locator('#transit-hours').textContent();await page.reload();await expect(page.locator('#transit-hours')).toHaveText(hours);
 const pending=page.waitForEvent('download');await page.locator('#download').click();const file=await pending;const json=JSON.parse(await fs.readFile(await file.path(),'utf8'));expect(json.scenario.transit_share_bps).toBe(5000);expect(json.scenario.transit_area).toBe(2);
 await page.locator('#lane-transportation').fill('-10');await expect(page.locator('#transit-hours')).toHaveText('0 vehicle revenue hours');
 await page.locator('#reset').click();await expect(page.locator('#transit-share')).toHaveValue('0');await expect(page.locator('#gap')).toHaveText('$1,774.684B');
});
