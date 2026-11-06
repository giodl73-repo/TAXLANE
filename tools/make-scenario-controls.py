from pathlib import Path
import json,copy,random
import argparse
parser=argparse.ArgumentParser(description='Generate frozen TAXLANE native/WASM controls')
parser.add_argument('baseline',type=Path)
parser.add_argument('destination',type=Path)
a=parser.parse_args()
d=a.destination
d.mkdir()
baseline_raw=a.baseline.read_text(encoding='utf-8')
(d/'baseline.json').write_text(baseline_raw,encoding='utf-8',newline='\n')
# Full raw JSON strings are passed to both targets; no JS number conversion.
def raw(value):return json.dumps(value,ensure_ascii=False,separators=(',',':'))
def scenario(**kw):return dict(lane_change_bps={},receipts_change_bps=0,transit_share_bps=0,transit_area=0)|kw
base=json.loads(baseline_raw);items=[]
def add(name,s=None,b=None,expect='ok',kind='portable',sraw=None,braw=None):items.append(dict(name=name,kind=kind,expect=expect,baseline=baseline_raw if b is None and braw is None else braw if braw is not None else raw(b),scenario=raw(scenario()if s is None else s)if sraw is None else sraw))
add('default')
lanes=[r['id']for r in base['lanes']if r['adjustable']]
for lane in lanes:
 for percent in range(-100,101):add(f'lane:{lane}:{percent}',scenario(lane_change_bps={lane:percent*100}),kind='ui-single-lane')
for percent in range(-100,101):add(f'receipts:{percent}',scenario(receipts_change_bps=percent*100),kind='ui-receipts');add(f'all-lanes:{percent}',scenario(lane_change_bps={k:percent*100 for k in lanes}),kind='ui-all-lanes')
for change in [100,1000,10000]:
 for area in range(4):
  for share in range(0,10001,100):add(f'transit:{change}:{area}:{share}',scenario(lane_change_bps={'transportation':change},transit_share_bps=share,transit_area=area),kind='ui-transit')
for change in [-10000,-100,0]:
 for area in range(4):
  for share in [0,100,5000,10000]:add(f'nonpositive-transit:{change}:{area}:{share}',scenario(lane_change_bps={'transportation':change},transit_share_bps=share,transit_area=area),kind='ui-transit-boundary')
for change in [1,50,99,100,101,999,1000,1001,9999,10000]:
 for area in range(4):
  for share in [0,1,50,99,100,101,4999,5000,9999,10000]:add(f'typed-transit:{change}:{area}:{share}',scenario(lane_change_bps={'transportation':change},transit_share_bps=share,transit_area=area),kind='typed-transit')
for n in [-10000,-9999,-9950,-9901,-9900,-51,-50,-49,-1,0,1,49,50,51,99,100,101,9999,10000]:
 for lane in lanes:add(f'typed-lane:{lane}:{n}',scenario(lane_change_bps={lane:n}),kind='typed-rounding')
 add(f'typed-receipts:{n}',scenario(receipts_change_bps=n),kind='typed-rounding')
# Deterministic fixture generation only; not a stochastic fiscal model.
generator=random.Random(20261106)
for i in range(256):add(f'mixed:{i}',scenario(lane_change_bps={k:generator.randrange(-100,101)*100 for k in lanes},receipts_change_bps=generator.randrange(-100,101)*100,transit_share_bps=generator.randrange(101)*100,transit_area=generator.randrange(4)),kind='ui-mixed')
for amount in [0,1,5,50,100,999999,9007199254740991,9007199254740993,9223372036854775807//3]:
 small=dict(version=1,fiscal_year=2025,receipts_musd=80,outlays_musd=amount+20,lanes=[dict(id='transportation',label='Synthetic rounding input',amount_musd=amount,adjustable=True,sources=[]),dict(id='net-interest',label='Fixed interest',amount_musd=20,adjustable=False,sources=[])])
 for n in [-10000,-9999,-5000,-50,-1,0,1,50,5000,9999,10000]:add(f'small-rounding:{amount}:{n}',scenario(lane_change_bps={'transportation':n},transit_share_bps=10000,transit_area=3),b=small,kind='typed-i64-rounding')
for lane in ['net-interest','commerce-housing-credit','undistributed-offsetting-receipts']:
 add('fixed-zero:'+lane,scenario(lane_change_bps={lane:0}),kind='accepted-native-boundary')
 for change in [-100,100]:add('fixed-reject:'+lane+':'+str(change),scenario(lane_change_bps={lane:change}),expect='error',kind='typed-invalid')
for key,values in [('receipts_change_bps',[-10001,10001,-2147483648,2147483647,1.5,None,True,'0']),('transit_share_bps',[10001,65535,65536,-1,1.5,None,True,'0']),('transit_area',[4,255,4294967295,-1,1.5,None,True,'0'])]:
 for value in values:add(f'invalid:{key}:{raw(value)}',scenario(**{key:value}),expect='error',kind='typed-invalid')
for value in [-10001,10001,-2147483648,2147483647,1.5,None,True,'0']:add('invalid-lane:'+raw(value),scenario(lane_change_bps={'transportation':value}),expect='error',kind='typed-invalid')
for s in [None,[],0,True,'x',{},dict(lane_change_bps={}),dict(receipts_change_bps=0),dict(lane_change_bps=[],receipts_change_bps=0),dict(lane_change_bps={'unknown':0},receipts_change_bps=0),scenario(extra=1)]:add('scenario-shape:'+raw(s),sraw=raw(s),expect='error',kind='json-invalid')
for text in ['', '{', 'null', '{"lane_change_bps":{},"receipts_change_bps":0,"receipts_change_bps":0}', '{"lane_change_bps":{"transportation":1e100},"receipts_change_bps":0}', '{"lane_change_bps":{},"receipts_change_bps":NaN}']:add('scenario-wire:'+text,sraw=text,expect='error',kind='json-invalid')
for key,values in [('version',[0,2,-1,4294967296,None,True]),('receipts_musd',[-1,9223372036854775808,1.5,None]),('fiscal_year',[-1,65536,None])]:
 for value in values:b=copy.deepcopy(base);b[key]=value;add(f'baseline:{key}:{raw(value)}',b=b,expect='error',kind='baseline-invalid')
for year in [0,65535]:b=copy.deepcopy(base);b['fiscal_year']=year;add('accepted-year:'+str(year),b=b,kind='accepted-native-boundary')
for field in ['version','fiscal_year','receipts_musd','outlays_musd','lanes']:
 b=copy.deepcopy(base);del b[field];add('missing-baseline:'+field,b=b,expect='error',kind='baseline-invalid')
for label,mutator in [('empty-lanes',lambda b:b.update(lanes=[])),('null-lane',lambda b:b['lanes'].__setitem__(0,None)),('empty-id',lambda b:b['lanes'][0].update(id='')),('duplicate-id',lambda b:b['lanes'][1].update(id=b['lanes'][0]['id'])),('bad-label',lambda b:b['lanes'][0].update(label=None)),('bad-sources',lambda b:b['lanes'][0].update(sources=[None])),('bad-adjustable',lambda b:b['lanes'][0].update(adjustable='true')),('sum-mismatch',lambda b:b.update(outlays_musd=b['outlays_musd']+1)),('missing-interest',lambda b:next(l for l in b['lanes'] if l['id']=='net-interest').update(id='renamed-interest')),('adjustable-interest',lambda b:next(l for l in b['lanes'] if l['id']=='net-interest').update(adjustable=True)),('extra-field',lambda b:b.update(extra=1))]:
 b=copy.deepcopy(base);mutator(b);add('baseline-'+label,b=b,expect='error',kind='baseline-invalid')
for text in ['', '{', '[]', 'null', 'true', '{"version":1,"version":1}']:add('baseline-wire:'+text,braw=text,expect='error',kind='json-invalid')
for label in ['Quote " and backslash \\','Newline\nTab\tUnicode ∧ 🍋','\u0000']:
 b=copy.deepcopy(base);b['lanes'][0]['label']=label;add('escaped-label:'+raw(label),b=b,kind='typed-text')
# usize-specific oversized JSON is kept separate, not counted as portable parity.
wide=[dict(name='usize-'+str(n),baseline=baseline_raw,scenario=raw(scenario(transit_area=n)))for n in [4294967296,18446744073709551615]]
(d/'controls.jsonl').write_bytes(b''.join((raw(c)+'\n').encode()for c in items));(d/'target-width-controls.jsonl').write_bytes(b''.join((raw(c)+'\n').encode()for c in wide));print(json.dumps(dict(controls=len(items),kinds={k:sum(c['kind']==k for c in items)for k in sorted({c['kind']for c in items})},separate_target_width_controls=len(wide))))
