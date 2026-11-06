import init, { compare_json } from './pkg/taxlane_scenario.js';
let baseline, baselineRaw;
self.onmessage = async ({data}) => {
  try {
    if (data.type === 'init') {
      await init();
      const response = await fetch('./baseline.json');
      if (!response.ok) throw new Error('Baseline download failed');
      baselineRaw = await response.text();
      baseline = JSON.parse(baselineRaw);
      self.postMessage({type:'ready', baseline});
    } else if (data.type === 'compare' && baseline) {
      const raw = compare_json(baselineRaw, JSON.stringify(data.scenario));
      self.postMessage({type:'result', id:data.id, raw});
    }
  } catch (error) {
    self.postMessage({type:'error', id:data?.id, fatal:data?.type==='init', message:String(error)});
  }
};
