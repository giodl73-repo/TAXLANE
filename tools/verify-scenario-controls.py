"""Compare complete native/WASM strings and independently check accounting fields."""
import argparse
from collections import Counter
import hashlib
import itertools
import json
import math
from pathlib import Path


def require(condition, message):
    if not condition:
        raise ValueError(message)


def scaled(amount, change):
    return (amount * (10000 + change) + 5000) // 10000


def accounting(control, response):
    baseline = json.loads(control['baseline'])
    scenario = json.loads(control['scenario'])
    result = json.loads(response)
    changes = scenario['lane_change_bps']
    lanes = []
    for lane in baseline['lanes']:
        before = lane['amount_musd']
        after = scaled(before, changes.get(lane['id'], 0)) if lane['adjustable'] else before
        lanes.append(dict(id=lane['id'], baseline_musd=before, scenario_musd=after,
                          change_musd=after-before))
    interest = next(l['scenario_musd'] for l in lanes if l['id'] == 'net-interest')
    outlays = sum(l['scenario_musd'] for l in lanes)
    receipts = scaled(baseline['receipts_musd'], scenario['receipts_change_bps'])
    original_deficit = baseline['outlays_musd'] - baseline['receipts_musd']
    expected = dict(fiscal_year=baseline['fiscal_year'],
                    baseline_outlays_musd=baseline['outlays_musd'], outlays_musd=outlays,
                    receipts_musd=receipts, primary_outlays_musd=outlays-interest,
                    net_interest_musd=interest, deficit_musd=outlays-receipts,
                    baseline_deficit_musd=original_deficit,
                    deficit_change_musd=outlays-receipts-original_deficit, lanes=lanes)
    require(set(result) == set(expected) | {'transit'}, 'Comparison field set changed')
    for key, value in expected.items():
        require(result[key] == value, 'Independent accounting differs: '+key)
    area = scenario.get('transit_area', 0)
    costs = [265.30, 226.93, 160.84, 534.97]
    areas = ['New York urbanized area', 'Next seven largest urbanized areas',
             'All other urbanized areas', 'Rural areas']
    increment = next((max(0, l['change_musd']) for l in lanes if l['id'] == 'transportation'), 0)
    allocation = float(increment) * scenario.get('transit_share_bps', 0) / 10000
    transit = result['transit']
    expected_transit = dict(allocation_musd=allocation,
        vehicle_revenue_hours=allocation*1000000/costs[area],
        operating_dollars_per_hour=costs[area], area=areas[area], price_year=2024,
        basis='Illustrative cost-equivalent; not predicted service',
        source_url='https://www.transit.dot.gov/sites/fta.dot.gov/files/2026-04/2024%20National%20Transit%20Summaries%20and%20Trends_1.2.pdf')
    require(set(transit) == set(expected_transit), 'Transit field set changed')
    for key, value in expected_transit.items():
        # Only this independent floating arithmetic check has a tolerance.
        # Native/WASM strings above are compared exactly, without rounding.
        require(math.isclose(transit[key], value, rel_tol=1e-14, abs_tol=1e-12)
                if isinstance(value, float) else transit[key] == value,
                'Independent transit check differs: '+key)


def verify(controls, native, wasm):
    counts = Counter()
    with controls.open(encoding='utf8') as c, native.open(encoding='utf8') as n, wasm.open(encoding='utf8') as w:
        for index, rows in enumerate(itertools.zip_longest(c, n, w), 1):
            require(all(row is not None for row in rows), 'Record count differs')
            control, original, browser = map(json.loads, rows)
            name = control['name']
            require(original == browser, f'Native/WASM string or error differs: {index} {name}')
            key = 'Ok' if control['expect'] == 'ok' else 'Err'
            require(set(original) == {key} and isinstance(original[key], str), 'Unexpected result: '+name)
            if key == 'Ok':
                accounting(control, original[key])
            else:
                require(bool(original[key]), 'Empty rejection: '+name)
            counts[key] += 1
            counts['total'] += 1
    require(counts['total'] > 0, 'No controls')
    return dict(counts=counts, files={p.name: dict(bytes=p.stat().st_size,
        sha256=hashlib.file_digest(p.open('rb'), 'sha256').hexdigest()) for p in [controls, native, wasm]},
        comparison='Complete Result strings/errors, with independent integer accounting and transit checks',
        scope='Portable typed inputs only; oversized usize target-width rejections are separate observations')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('controls', type=Path)
    parser.add_argument('native', type=Path)
    parser.add_argument('wasm', type=Path)
    parser.add_argument('--receipt', type=Path)
    args = parser.parse_args()
    receipt = verify(args.controls, args.native, args.wasm)
    text = json.dumps(receipt, indent=2)+'\n'
    if args.receipt:
        with args.receipt.open('x', encoding='utf8') as output:
            output.write(text)
    print(text)
