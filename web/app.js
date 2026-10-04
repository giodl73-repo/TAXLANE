const $ = selector => document.querySelector(selector);
const money = value => `$${(Math.abs(value)/1000).toLocaleString('en-US',{minimumFractionDigits:1,maximumFractionDigits:3})}B`;
const percent = value => `${value > 0 ? '+' : ''}${value}%`;
let baseline, ready=false, request=0, timer;
let scenario={lane_change_bps:{},receipts_change_bps:0};
const controls=new Map();
const worker=new Worker(new URL('./worker.js',import.meta.url),{type:'module'});
function fail(message='Budget explorer could not load. Reload to retry, or use the research link above.') {
  ready=false; $('#status').textContent=message;
  document.querySelectorAll('button,input').forEach(control=>control.disabled=true);
}
function changed() {
  request++;
  if(!ready) return;
  clearTimeout(timer);
  timer=setTimeout(()=>worker.postMessage({type:'compare',id:request,scenario}),60);
}
function syncControls() {
  for(const [id,{input,output}] of controls) {
    input.value=(scenario.lane_change_bps[id]||0)/100;
    output.textContent=percent(Number(input.value));
  }
  $('#receipts').value=scenario.receipts_change_bps/100;
  $('#receipts-value').textContent=percent(Number($('#receipts').value));
}
function loadScenario() {
  const raw=new URLSearchParams(location.search).get('scenario');
  if(!raw) return;
  try {
    if(raw.length>5000) throw new Error();
    const candidate=JSON.parse(raw);
    if(!candidate || Object.keys(candidate).some(key=>!['lane_change_bps','receipts_change_bps'].includes(key)) || !candidate.lane_change_bps || typeof candidate.lane_change_bps!=='object' || Array.isArray(candidate.lane_change_bps)) throw new Error();
    const valid=value=>Number.isInteger(value) && value>=-10000 && value<=10000 && value%100===0;
    if(!valid(candidate.receipts_change_bps) || Object.entries(candidate.lane_change_bps).some(([id,value])=>!controls.has(id)||!valid(value))) throw new Error();
    scenario=candidate;
  } catch { $('#status').textContent='Invalid shared scenario; showing the baseline.'; }
}
worker.onerror=()=>fail();
worker.onmessage=({data})=>{
  if(data.type==='error') {fail();return;}
  if(data.type==='ready') {
    baseline=data.baseline;
    $('#baseline-label').textContent=`FY${baseline.fiscal_year} · OMB FY2027 source vintage · draft research ledger`;
    for(const lane of baseline.lanes.filter(lane=>lane.adjustable)) {
      const article=document.createElement('article');article.className='lane';
      const head=document.createElement('div');head.className='lane-head';
      const label=document.createElement('label');label.htmlFor=`lane-${lane.id}`;label.textContent=lane.label;
      const output=document.createElement('output');output.htmlFor=label.htmlFor;output.textContent='0%';
      head.append(label,output);
      const input=document.createElement('input');Object.assign(input,{type:'range',min:'-100',max:'100',step:'1',value:'0',id:label.htmlFor});
      const amount=document.createElement('p');amount.className='amount';amount.id=`amount-${lane.id}`;amount.textContent=`Baseline ${money(lane.amount_musd)}`;
      input.setAttribute('aria-describedby',amount.id);
      const source=document.createElement('a');source.href='https://github.com/giodl73-repo/TAXLANE/blob/main/docs/jurisdiction-map.md';source.textContent='Inspect purpose, service owner, and evidence ↗';
      input.oninput=()=>{scenario.lane_change_bps[lane.id]=Number(input.value)*100;output.textContent=percent(Number(input.value));changed();};
      controls.set(lane.id,{input,output});article.append(head,input,amount,source);$('#lanes').append(article);
    }
    ready=true;document.querySelectorAll('button,input').forEach(control=>control.disabled=false);
    $('#status').textContent='Ready · calculations stay on your device';
    loadScenario();syncControls();changed();
  } else if(data.type==='result' && data.id===request) {
    const result=data.result;
    $('#outlays').textContent=money(result.outlays_musd);$('#revenue').textContent=money(result.receipts_musd);
    $('#gap-label').textContent=result.deficit_musd<0?'Annual surplus':'Annual financing gap';$('#gap').textContent=money(result.deficit_musd);
    $('#mobile-gap').textContent=`${money(result.deficit_musd)} ${result.deficit_musd<0?'surplus':'gap'}`;
    $('#delta').textContent=result.deficit_change_musd===0?'Same financing gap as baseline.':`${money(result.deficit_change_musd)} ${result.deficit_change_musd<0?'less':'more'} financing required than baseline.`;
    const coverage=result.outlays_musd>0?100*result.receipts_musd/result.outlays_musd:null;
    $('#coverage-bar').style.width=`${coverage===null?0:Math.min(100,Math.max(0,coverage))}%`;
    $('#coverage').textContent=coverage===null?'Receipt coverage is undefined for nonpositive net outlays.':`Receipts cover ${coverage.toFixed(1)}% of scenario spending.`;
    $('#changes').replaceChildren();
    for(const lane of result.lanes) {
      const amount=$(`#amount-${lane.id}`);if(amount) amount.textContent=`${money(lane.scenario_musd)} scenario · ${money(lane.baseline_musd)} baseline`;
      if(lane.change_musd!==0) {const li=document.createElement('li');li.textContent=`${baseline.lanes.find(item=>item.id===lane.id).label}: ${money(lane.change_musd)} ${lane.change_musd<0?'less':'more'} funding.`;$('#changes').append(li);}
    }
    if(!$('#changes').children.length) {const li=document.createElement('li');li.textContent='Spending priorities match the baseline.';$('#changes').append(li);}
    $('#fixed-accounting').textContent=`Net interest: ${money(result.net_interest_musd)}. Negative offsets: ${money(baseline.lanes.filter(lane=>lane.amount_musd<0).reduce((sum,lane)=>sum+lane.amount_musd,0))} reduction in net outlays.`;
  }
};
$('#receipts').oninput=()=>{scenario.receipts_change_bps=Number($('#receipts').value)*100;$('#receipts-value').textContent=percent(Number($('#receipts').value));changed();};
$('#reset').onclick=()=>{scenario={lane_change_bps:{},receipts_change_bps:0};history.replaceState(null,'',location.pathname);syncControls();$('#status').textContent='Reset to baseline';changed();};
$('#share').onclick=async()=>{const url=new URL(location.href);url.searchParams.set('scenario',JSON.stringify(scenario));history.replaceState(null,'',url);try{await navigator.clipboard.writeText(url.href);$('#status').textContent='Scenario link copied.';}catch{$('#status').textContent='Copy the scenario link from your address bar.';}};
$('#download').onclick=()=>{const blob=new Blob([JSON.stringify({baseline_version:baseline.version,fiscal_year:baseline.fiscal_year,hypothetical:true,scenario},null,2)],{type:'application/json'});const url=URL.createObjectURL(blob);const link=document.createElement('a');link.href=url;link.download='taxlane-scenario.json';link.click();setTimeout(()=>URL.revokeObjectURL(url),1000);};
worker.postMessage({type:'init'});
