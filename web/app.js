import {evidencePanel} from './lane-evidence.js';
const $ = selector => document.querySelector(selector);
const money = value => `$${(Math.abs(value)/1000).toLocaleString('en-US',{minimumFractionDigits:1,maximumFractionDigits:3})}B`;
const percent = value => `${value > 0 ? '+' : ''}${value}%`;
let baseline, worker, ready=false, request=0, timer, loadingTimer, calculationTimer, attempt=0, scenarioLoaded=false;
let scenario={lane_change_bps:{},receipts_change_bps:0,transit_share_bps:0,transit_area:0};
const controls=new Map();

function openEvidenceHash() {
  const target=document.getElementById(location.hash.slice(1));
  if(target?.classList.contains('lane-evidence')) {target.open=true;target.scrollIntoView({block:'start'});}
}
addEventListener('hashchange',openEvidenceHash);
function clearTimers() {clearTimeout(timer);clearTimeout(loadingTimer);clearTimeout(calculationTimer);}
function clearResults() {
  $('.summary').hidden=true;$('#share').disabled=true;$('#download').disabled=true;
  for(const id of ['outlays','revenue','gap','mobile-gap','transit-hours']) $('#'+id).textContent='—';
  for(const id of ['delta','coverage','fixed-accounting','transit-detail']) $('#'+id).textContent='';
  $('#changes').replaceChildren();$('#coverage-bar').style.width='0%';
  if(Array.isArray(baseline?.lanes))for(const lane of baseline.lanes){const amount=$('#amount-'+lane.id);if(amount)amount.textContent=`Baseline ${money(lane.amount_musd)}`;}
}
function disableControls() {document.querySelectorAll('button,input,select').forEach(control=>control.disabled=true);}
function fail(message='Budget explorer could not load.') {
  clearTimers();attempt++;ready=false;worker?.terminate();clearResults();disableControls();
  if(!baseline)$('#baseline-label').textContent='Dated baseline unavailable';
  $('#status').textContent=message+' Retry the explorer when you are ready, or use the research link above.';
  $('#retry').hidden=false;$('#retry').disabled=false;
}
function changed() {
  request++;clearTimeout(timer);clearTimeout(calculationTimer);
  if(!ready) return;
  clearResults();
  const id=request,generation=attempt;
  timer=setTimeout(()=>{
    if(generation!==attempt||!ready)return;
    calculationTimer=setTimeout(()=>{if(generation===attempt&&id===request)fail('Budget calculation timed out.');},10000);
    try{worker.postMessage({type:'compare',id,scenario});}catch{fail();}
  },60);
}
function syncControls() {
  for(const [id,{input,output}] of controls) {
    input.value=(scenario.lane_change_bps[id]||0)/100;
    output.textContent=percent(Number(input.value));
  }
  $('#transit-share').value=scenario.transit_share_bps/100;$('#transit-area').value=scenario.transit_area;$('#transit-share-value').textContent=(scenario.transit_share_bps/100)+'%';
  $('#receipts').value=scenario.receipts_change_bps/100;
  $('#receipts-value').textContent=percent(Number($('#receipts').value));
}
function loadScenario() {
  const raw=new URLSearchParams(location.search).get('scenario');
  if(!raw) return;
  try {
    if(raw.length>5000) throw new Error();
    const candidate=JSON.parse(raw);
    if(!candidate || Object.keys(candidate).some(key=>!['lane_change_bps','receipts_change_bps','transit_share_bps','transit_area'].includes(key)) || !candidate.lane_change_bps || typeof candidate.lane_change_bps!=='object' || Array.isArray(candidate.lane_change_bps)) throw new Error();
    const valid=value=>Number.isInteger(value) && value>=-10000 && value<=10000 && value%100===0;
    if(!valid(candidate.receipts_change_bps) || Object.entries(candidate.lane_change_bps).some(([id,value])=>!controls.has(id)||!valid(value))) throw new Error();
    candidate.transit_share_bps??=0;candidate.transit_area??=0;
    if(!Number.isInteger(candidate.transit_share_bps)||candidate.transit_share_bps<0||candidate.transit_share_bps>10000||candidate.transit_share_bps%100!==0||!Number.isInteger(candidate.transit_area)||candidate.transit_area<0||candidate.transit_area>3)throw Error();
    scenario=candidate;
  } catch { $('#status').textContent='Invalid shared scenario; showing the baseline.'; }
}
function receive(data) {
  if(data.type==='error') {fail();return;}
  if(data.type==='ready') {
    if(ready)return;
    clearTimeout(loadingTimer);
    const openPanels=new Set([...document.querySelectorAll('.lane-evidence[open]')].map(panel=>panel.id));
    const candidate=data.baseline;
    if(!candidate||!Array.isArray(candidate.lanes)||candidate.lanes.some(lane=>!lane||typeof lane.id!=='string'||typeof lane.label!=='string'||!Number.isFinite(lane.amount_musd)))throw Error('Invalid baseline response');
    baseline=candidate;
    controls.clear();$('#lanes').replaceChildren();
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
      input.oninput=()=>{scenario.lane_change_bps[lane.id]=Number(input.value)*100;output.textContent=percent(Number(input.value));changed();};
      controls.set(lane.id,{input,output});article.append(head,input,amount,evidencePanel(lane));$('#lanes').append(article);
    }
    $('#fixed-evidence').replaceChildren(...baseline.lanes.filter(lane=>!lane.adjustable).map(evidencePanel));
    for(const id of openPanels){const panel=document.getElementById(id);if(panel)panel.open=true;}
    openEvidenceHash();
    ready=true;document.querySelectorAll('button,input,select').forEach(control=>control.disabled=false);
    $('#status').textContent='Ready · calculations stay on your device';
    if(!scenarioLoaded){loadScenario();scenarioLoaded=true;}syncControls();changed();
    $('#retry').hidden=true;$('#retry').disabled=true;
  } else if(ready && data.type==='result' && data.id===request) {
    clearTimeout(calculationTimer);
    const result=data.result;const t=result.transit;$('#transit-hours').textContent=Math.round(t.vehicle_revenue_hours).toLocaleString('en-US')+' vehicle revenue hours';$('#transit-detail').textContent=`$${t.allocation_musd.toLocaleString('en-US',{maximumFractionDigits:3})}M illustrative allocation ÷ $${t.operating_dollars_per_hour.toFixed(2)}/hour (${t.area}, ${t.price_year} dollars).`;
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
    $('.summary').hidden=false;$('#share').disabled=false;$('#download').disabled=false;
  }
}
function start() {
  clearTimers();const generation=++attempt;ready=false;worker?.terminate();clearResults();disableControls();
  $('#retry').hidden=true;$('#status').textContent='Loading Rust accounting engine…';
  if(!baseline)$('#baseline-label').textContent='Loading the dated baseline…';
  try{
    worker=new Worker(new URL('./worker.js',import.meta.url),{type:'module'});
    loadingTimer=setTimeout(()=>{if(generation===attempt)fail('Budget engine loading timed out.');},15000);
    worker.onerror=()=>{if(generation===attempt)fail();};
    worker.onmessage=({data})=>{if(generation!==attempt)return;try{receive(data);}catch{fail('Invalid budget response.');}};
    worker.postMessage({type:'init'});
  }catch{fail('Unable to start the budget engine.');}
}
$('#retry').onclick=start;
$('#transit-share').oninput=()=>{scenario.transit_share_bps=Number($('#transit-share').value)*100;$('#transit-share-value').textContent=$('#transit-share').value+'%';changed();};
$('#transit-area').onchange=()=>{scenario.transit_area=Number($('#transit-area').value);changed();};
$('#transit-example').onclick=()=>{scenario.lane_change_bps.transportation=1000;scenario.transit_share_bps=1000;scenario.transit_area=2;syncControls();changed();};
$('#receipts').oninput=()=>{scenario.receipts_change_bps=Number($('#receipts').value)*100;$('#receipts-value').textContent=percent(Number($('#receipts').value));changed();};
$('#reset').onclick=()=>{scenario={lane_change_bps:{},receipts_change_bps:0,transit_share_bps:0,transit_area:0};history.replaceState(null,'',location.pathname);syncControls();$('#status').textContent='Reset to baseline';changed();};
$('#share').onclick=async()=>{const url=new URL(location.href);url.searchParams.set('scenario',JSON.stringify(scenario));history.replaceState(null,'',url);try{await navigator.clipboard.writeText(url.href);$('#status').textContent='Scenario link copied.';}catch{$('#status').textContent='Copy the scenario link from your address bar.';}};
$('#download').onclick=()=>{const blob=new Blob([JSON.stringify({baseline_version:baseline.version,fiscal_year:baseline.fiscal_year,hypothetical:true,scenario},null,2)],{type:'application/json'});const url=URL.createObjectURL(blob);const link=document.createElement('a');link.href=url;link.download='taxlane-scenario.json';link.click();setTimeout(()=>URL.revokeObjectURL(url),1000);};
start();
