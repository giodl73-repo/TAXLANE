import {test,expect} from '@playwright/test';
import {readFileSync} from 'node:fs';
const base='/TAXLANE/';
const shared={lane_change_bps:{health:-1000},receipts_change_bps:0,transit_share_bps:0,transit_area:0};
async function exported(page){const pending=page.waitForEvent('download');await page.locator('#download').click();return JSON.parse(readFileSync(await(await pending).path(),'utf8'));}
async function track(page){await page.addInitScript(()=>{const NativeWorker=window.Worker;window.__workers=[];window.Worker=class extends NativeWorker{constructor(...args){super(...args);window.__workers.push(this);}postMessage(data,...args){if(data.type==='compare')this.__id=data.id;return super.postMessage(data,...args);}};});}

test('malformed baseline rows fail safely and retry the actual dated ledger',async({page})=>{
 let malformed=true;const errors=[];page.on('pageerror',e=>errors.push(e.message));
 await page.route('**/baseline.json',r=>malformed?r.fulfill({status:200,contentType:'application/json',body:'{"lanes":[null]}'}):r.continue());
 await page.goto(base);await expect(page.locator('#retry')).toBeVisible();await expect(page.locator('#status')).toContainText('Invalid budget response');await expect(page.locator('#gap')).toHaveText('—');
 malformed=false;await page.locator('#retry').click();await expect(page.locator('#gap')).toHaveText('$1,774.684B');await expect(page.locator('.lane-evidence')).toHaveCount(17);expect(errors).toEqual([]);
});

test('baseline 503 retries by keyboard and loads the shared scenario once',async({page})=>{
 let fail=true;await page.route('**/baseline.json',r=>fail?r.fulfill({status:503,body:'Unavailable'}):r.continue());
 await page.setViewportSize({width:390,height:844});await page.goto(base+'?scenario='+encodeURIComponent(JSON.stringify(shared)));
 await expect(page.locator('#retry')).toBeVisible();await expect(page.locator('#baseline-label')).toHaveText('Dated baseline unavailable');await expect(page.locator('#gap')).toHaveText('—');await expect(page.locator('#download')).toBeDisabled();
 fail=false;await page.locator('#retry').focus();await page.keyboard.press('Enter');await expect(page.locator('#gap')).toHaveText('$1,676.833B');
 expect((await exported(page)).scenario).toEqual(shared);await expect(page.locator('.lane')).toHaveCount(14);await expect(page.locator('.lane-evidence')).toHaveCount(17);
 expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
});

test('retry preserves changes newer than the shared URL and Reset remains explicit',async({page})=>{
 await track(page);await page.goto(base+'?scenario='+encodeURIComponent(JSON.stringify(shared)));await expect(page.locator('#gap')).toHaveText('$1,676.833B');
 await page.locator('#transit-example').click();await expect(page.locator('#transit-detail')).toContainText('$160.84/hour');
 await page.locator('#receipts').fill('2');await expect(page.locator('#download')).toBeEnabled();const before=await exported(page);const gap=await page.locator('#gap').textContent();
 await page.locator('#evidence-health summary').click();await page.evaluate(()=>{const w=window.__workers.at(-1);window.__oldReply=w.onmessage;w.onerror(new Event('error'));});
 await expect(page.locator('.summary')).toBeHidden();await expect(page.locator('#gap')).toHaveText('—');await expect(page.locator('#transit-hours')).toHaveText('—');await expect(page.locator('#share')).toBeDisabled();
 await page.locator('#retry').click();await expect(page.locator('#gap')).toHaveText(gap);expect(await exported(page)).toEqual(before);
 await expect(page.locator('#evidence-health')).toHaveAttribute('open','');await expect(page.locator('.lane')).toHaveCount(14);await expect(page.locator('.lane-evidence')).toHaveCount(17);
 await page.evaluate(()=>window.__oldReply({data:{type:'ready',baseline:null}}));await expect(page.locator('#gap')).toHaveText(gap);
 await page.locator('#reset').click();await expect(page.locator('#gap')).toHaveText('$1,774.684B');expect((await exported(page)).scenario.lane_change_bps).toEqual({});
});

test('WASM download failure retries actual accounting with public routes intact',async({page})=>{
 await page.route('**/*.wasm',r=>r.abort());await page.goto(base);await expect(page.locator('#retry')).toBeVisible();await expect(page.locator('#reset')).toBeDisabled();
 await expect(page.getByRole('link',{name:'Research & sources'})).toBeVisible();expect((await page.request.get(base+'provenance.json')).ok()).toBe(true);expect((await page.request.get(base+'LICENSE')).ok()).toBe(true);
 await page.unrouteAll({behavior:'wait'});await page.locator('#retry').click();await expect(page.locator('#gap')).toHaveText('$1,774.684B');
});

test('stalled initialization has a bounded deadline and in-page recovery',async({page})=>{
 await page.addInitScript(()=>{const NativeWorker=window.Worker,nativeTimer=window.setTimeout.bind(window);window.__stall=true;window.Worker=class extends NativeWorker{postMessage(data,...args){if(window.__stall&&data.type==='init')return;return super.postMessage(data,...args);}};window.setTimeout=(fn,ms,...args)=>nativeTimer(fn,window.__stall&&ms===15000?2000:ms,...args);});
 await page.goto(base);await expect(page.locator('#status')).toContainText('Budget engine loading timed out');await expect(page.locator('#share')).toBeDisabled();await expect(page.locator('#gap')).toHaveText('—');
 await page.evaluate(()=>{window.__stall=false;});await page.locator('#retry').click();await expect(page.locator('#gap')).toHaveText('$1,774.684B');
});

test('calculation timeout clears stale totals and retries the current assumption',async({page})=>{
 await page.addInitScript(()=>{const NativeWorker=window.Worker,nativeTimer=window.setTimeout.bind(window);window.__stall=false;window.Worker=class extends NativeWorker{postMessage(data,...args){if(window.__stall&&data.type==='compare')return;return super.postMessage(data,...args);}};window.setTimeout=(fn,ms,...args)=>nativeTimer(fn,window.__stall&&ms===10000?1000:ms,...args);});
 await page.goto(base);await expect(page.locator('#gap')).toHaveText('$1,774.684B');await page.evaluate(()=>{window.__stall=true;});await page.locator('#lane-health').fill('-10');
 await expect(page.locator('#status')).toContainText('Budget calculation timed out');await expect(page.locator('#gap')).toHaveText('—');await expect(page.locator('#download')).toBeDisabled();
 await page.evaluate(()=>{window.__stall=false;});await page.locator('#retry').click();await expect(page.locator('#gap')).toHaveText('$1,676.833B');expect((await exported(page)).scenario.lane_change_bps.health).toBe(-1000);
});

test('constructor failure stays readable and retries without an uncaught page error',async({page})=>{
 const errors=[];page.on('pageerror',e=>errors.push(e.message));await page.addInitScript(()=>{const NativeWorker=window.Worker;window.__reject=true;window.Worker=class extends NativeWorker{constructor(...args){if(window.__reject)throw Error('Worker construction blocked');super(...args);}};});
 await page.goto(base);await expect(page.locator('#status')).toContainText('Unable to start the budget engine');await expect(page.locator('#retry')).toBeVisible();
 await page.evaluate(()=>{window.__reject=false;});await page.locator('#retry').click();await expect(page.locator('#gap')).toHaveText('$1,774.684B');expect(errors).toEqual([]);
});

test('obsolete results are ignored and malformed current output requires fresh accounting',async({page})=>{
 await track(page);await page.goto(base);await expect(page.locator('#gap')).toHaveText('$1,774.684B');
 await page.evaluate(()=>{const w=window.__workers.at(-1);w.onmessage({data:{type:'result',id:w.__id-1,result:{}}});});await expect(page.locator('#gap')).toHaveText('$1,774.684B');
 await page.evaluate(()=>{const w=window.__workers.at(-1);w.onmessage({data:{type:'result',id:w.__id,result:{}}});});await expect(page.locator('#status')).toContainText('Invalid budget response');await expect(page.locator('#gap')).toHaveText('—');await expect(page.locator('.summary')).toBeHidden();
 await page.locator('#retry').click();await expect(page.locator('#gap')).toHaveText('$1,774.684B');await expect(page.locator('.lane-evidence')).toHaveCount(17);
});
