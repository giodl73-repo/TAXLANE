import {test,expect} from '@playwright/test';

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
