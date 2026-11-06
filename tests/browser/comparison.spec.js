import {test,expect} from '@playwright/test';
import {readFileSync} from 'node:fs';

async function track(page) {
  await page.addInitScript(()=>{
    const NativeWorker=window.Worker;
    window.__workers=[];
    window.Worker=class extends NativeWorker {
      constructor(...args) {
        super(...args);window.__workers.push(this);
        this.addEventListener('message',({data})=>{if(data.type==='result')this.__result=data;});
      }
      postMessage(data,...args) {
        if(data.type==='compare')this.__id=data.id;
        return super.postMessage(data,...args);
      }
    };
  });
}
async function download(page, id) {
  const pending=page.waitForEvent('download');
  await page.locator(id).click();
  const file=await pending;
  return {name:file.suggestedFilename(),text:readFileSync(await file.path(),'utf8')};
}

test('comparison inspection and download preserve the complete actual Rust string',async({page})=>{
  await track(page);await page.goto('/TAXLANE/');
  await expect(page.locator('#download-comparison')).toBeEnabled();
  await page.locator('#transit-example').click();
  await expect(page.locator('#transit-detail')).toContainText('$160.84/hour');
  await expect(page.locator('#download-comparison')).toBeEnabled();
  const raw=await page.evaluate(()=>window.__workers.at(-1).__result.raw);
  const output=JSON.parse(raw);
  expect(output.lanes).toHaveLength(17);
  expect(output.transit.price_year).toBe(2024);
  expect(output.transit.basis).toBe('Illustrative cost-equivalent; not predicted service');
  expect(output.transit.vehicle_revenue_hours).not.toBe(Math.round(output.transit.vehicle_revenue_hours));
  await page.locator('#raw-comparison summary').focus();await page.keyboard.press('Enter');
  expect(await page.locator('#raw-json').textContent()).toBe(raw);
  const file=await download(page,'#download-comparison');
  expect(file.name).toBe('taxlane-comparison.json');expect(file.text).toBe(raw);
  const input=JSON.parse((await download(page,'#download')).text);
  expect(Object.keys(input).sort()).toEqual(['baseline_version','fiscal_year','hypothetical','scenario']);
  expect(input.scenario.lane_change_bps.transportation).toBe(1000);
  expect(input.scenario.transit_share_bps).toBe(1000);
  expect(input.scenario.transit_area).toBe(2);
  await page.setViewportSize({width:390,height:844});
  expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
});

test('keyboard Stop clears raw results and Retry retains assumptions and evidence',async({page})=>{
  await track(page);await page.goto('/TAXLANE/');
  await expect(page.locator('#gap')).toHaveText('$1,774.684B');
  await page.locator('#lane-health').fill('-10');
  await expect(page.locator('#gap')).toHaveText('$1,676.833B');
  const before=(await download(page,'#download')).text;
  await page.locator('#evidence-health summary').click();
  await page.evaluate(()=>{const w=window.__workers.at(-1);window.__oldReply=w.onmessage;window.__oldResult=w.__result;});
  await page.locator('#stop').focus();await page.keyboard.press('Enter');
  await expect(page.locator('#status')).toContainText('Budget explorer stopped');
  await expect(page.locator('#raw-comparison')).toBeHidden();
  await expect(page.locator('#raw-json')).toHaveText('');
  await expect(page.locator('#download-comparison')).toBeDisabled();
  await expect(page.locator('#download')).toBeDisabled();
  await page.evaluate(()=>window.__oldReply({data:window.__oldResult}));
  await expect(page.locator('#gap')).toHaveText('—');
  await page.locator('#retry').focus();await page.keyboard.press('Enter');
  await expect(page.locator('#gap')).toHaveText('$1,676.833B');
  await expect(page.locator('#evidence-health')).toHaveAttribute('open','');
  expect((await download(page,'#download')).text).toBe(before);
  await expect(page.locator('#download-comparison')).toBeEnabled();
});

test('late rejected requests cannot retire a newer comparison or restore cleared output',async({page})=>{
  await track(page);await page.goto('/TAXLANE/');
  await expect(page.locator('#download-comparison')).toBeEnabled();
  const raw=await page.locator('#raw-json').textContent();
  await page.evaluate(()=>{const w=window.__workers.at(-1);w.onmessage({data:{type:'error',id:w.__id-1,fatal:false,message:'obsolete rejection'}});});
  await expect(page.locator('#gap')).toHaveText('$1,774.684B');
  expect(await page.locator('#raw-json').textContent()).toBe(raw);
  await expect(page.locator('#download-comparison')).toBeEnabled();
  await page.evaluate(()=>{const w=window.__workers.at(-1);w.onmessage({data:{type:'error',id:w.__id,fatal:false,message:'current rejection'}});});
  await expect(page.locator('#retry')).toBeVisible();
  await expect(page.locator('#raw-comparison')).toBeHidden();
  await expect(page.locator('#raw-json')).toHaveText('');
  await expect(page.locator('#download-comparison')).toBeDisabled();
  await page.locator('#retry').click();
  await expect(page.locator('#download-comparison')).toBeEnabled();
  expect(await page.locator('#raw-json').textContent()).toBe(raw);
});
