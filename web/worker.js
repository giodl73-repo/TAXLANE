import init, { compare_json } from './pkg/taxlane_scenario.js';
let baseline;
self.onmessage = async ({data}) => {
  try {
    if (data.type === 'init') {
      await init();
      const response = await fetch('./baseline.json');
      if (!response.ok) throw new Error('Baseline download failed');
      baseline = await response.json();
      self.postMessage({type:'ready', baseline});
    } else if (data.type === 'compare' && baseline) {
      const result = JSON.parse(compare_json(JSON.stringify(baseline), JSON.stringify(data.scenario)));
      self.postMessage({type:'result', id:data.id, result});
    }
  } catch (error) { self.postMessage({type:'error', message:String(error)}); }
};
