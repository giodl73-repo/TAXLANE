import {test,expect} from '@playwright/test';

test('no-JavaScript guide names the complete source and exact grants for this runtime',async({browser,baseURL})=>{
  const context=await browser.newContext({baseURL,javaScriptEnabled:false,viewport:{width:390,height:844}});
  const page=await context.newPage();
  try {
    await page.goto('/TAXLANE/build.html');
    const response=await page.request.get('/TAXLANE/BUILD-MANIFEST.json');
    expect(response.ok()).toBe(true);
    const manifest=await response.json();
    expect(manifest.schema).toBe('taxlane.pages-build.v1');
    expect(manifest.commit).toMatch(/^[0-9a-f]{40}$/);
    expect(manifest.source_artifacts.commit).toBe(manifest.commit);
    expect(manifest.source_artifacts.tree).toBe(manifest.tree);
    expect(manifest.source_artifacts.source_files).toBeGreaterThanOrEqual(3073);
    expect(manifest.source_artifacts.source_bytes).toBeGreaterThanOrEqual(419974005);
    const archive=manifest.source_release.files.find(f=>f.path==='source.tar.xz');
    await expect(page.locator('#source-download')).toHaveAttribute('href',manifest.source_release.base_url+'source.tar.xz');
    await expect(page.locator('#source-sha')).toHaveText(archive.sha256);
    await expect(page.locator('main')).toContainText(String(archive.bytes)+' bytes');
    await expect(page.locator('#receiver-download')).toHaveAttribute('href',manifest.source_release.base_url+'receiver.zip');
    await expect(page.locator('main')).toContainText('--inputs ../new-rust-inputs');
    await expect(page.locator('main')).toContainText('Third-party/source-specific data retains its own original terms');
    for(const name of ['LICENSE','DEPENDENCY-NOTICES.txt','RUST-RUNTIME-NOTICES.html','COMPONENT-NOTICES.txt']) {
      const file=await page.request.get('/TAXLANE/'+name);expect(file.ok()).toBe(true);
      expect((await file.body()).length).toBeGreaterThan(1000);
    }
    expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
  } finally {await context.close();}
});

test('actual browser worker returns complete published native comparison examples',async({page})=>{
  await page.goto('/TAXLANE/');
  await expect(page.locator('#download-comparison')).toBeEnabled();
  const comparisons=await page.evaluate(async()=>{
    const manifest=await(await fetch('BUILD-MANIFEST.json')).json();
    const examples=(await(await fetch('native-examples.ndjson')).text()).trim().split('\n').map(JSON.parse);
    return await new Promise((resolve,reject)=>{
      const worker=new Worker(new URL(manifest.runtime+'/worker.js',location.href),{type:'module'});
      const output=[];let index=0;
      const finish=(error)=>{clearTimeout(timer);worker.terminate();error?reject(error):resolve(output);};
      const timer=setTimeout(()=>finish(Error('Native example worker deadline')),15000);
      worker.onerror=()=>finish(Error('Native example worker failure'));
      worker.onmessage=({data})=>{
        if(data.type==='error')return finish(Error(data.message));
        if(data.type==='ready') {
          if(JSON.stringify(data.baseline)!==JSON.stringify(JSON.parse(examples[0].inputs.baseline)))return finish(Error('Worker/native baseline differs'));
        } else if(data.type==='result') {
          if(data.id!==index)return finish(Error('Unexpected example request'));
          output.push({native:examples[index].native.Ok,wasm:data.raw});index++;
          if(index===examples.length)return finish();
        } else return;
        worker.postMessage({type:'compare',id:index,scenario:JSON.parse(examples[index].inputs.scenario)});
      };
      worker.postMessage({type:'init'});
    });
  });
  expect(comparisons).toHaveLength(3);
  for(const response of comparisons)expect(response.wasm).toBe(response.native);
});
