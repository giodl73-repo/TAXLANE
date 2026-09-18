"""Capture PennDOT RMS attributes for county-route pairs in the rumble workbook."""

from __future__ import annotations

import argparse
import json
import time
import urllib.parse
import urllib.request
from collections import defaultdict
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path

import openpyxl


BASE = "https://gis.penndot.pa.gov/gis/rest/services/opendata"
SERVICES = {
    "roadwaysegments": [
        "OBJECTID", "ST_RT_NO", "CTY_CODE", "DISTRICT_NO", "JURIS",
        "SEG_NO", "SEG_LNGTH_FEET", "YR_BUILT", "YR_RESURF", "DIR_IND",
        "FAC_TYPE", "TOTAL_WIDTH", "SURF_TYPE", "LANE_CNT", "DIVSR_TYPE",
        "COND_DATE", "ROUGH_INDX", "PVMNT_COND_RATE", "CUR_AADT",
        "MAINT_RESPON_IND", "URBAN_RURAL", "NORM_ADMIN_BGN", "NORM_SHLD_BGN",
        "OVERALL_PVMNT_IDX", "SEG_STATUS", "BIKE_LANE", "GOVT_LVL_CTRL",
        "IRI_YEAR", "OPI_YEAR", "IRI_RATING_TEXT", "OPI_RATING_TEXT",
        "SURFACE_YEAR", "SEGMENT_MILES", "SHLD_COND_STATUS",
    ],
    "roadwayadmin": [
        "OBJECTID", "ST_RT_NO", "CTY_CODE", "DISTRICT_NO", "JURIS",
        "SEG_BGN", "OFFSET_BGN", "SEG_END", "OFFSET_END", "SEG_LNGTH_FEET",
        "MAINT_FUNC_CLS", "SPEED_LIMIT", "FED_AID_SYS", "FED_AID_URBAN_AREA",
        "FUNC_CLS", "FHWA_FUNC_CLS", "RECORD_UPDATE",
    ],
    "roadwayshoulder": [
        "OBJECTID", "ST_RT_NO", "CTY_CODE", "DISTRICT_NO", "JURIS",
        "SEG_BGN", "OFFSET_BGN", "SEG_END", "OFFSET_END", "SEG_LNGTH_FEET",
        "COND_DATE", "SHLD_SIDE_IND", "SHLD_TYPE", "SHLD_PAVE_WIDTH",
        "SHLD_TRTMT_DATA", "CURRENT_SHLD_TYPE", "CURRENT_PAVE_WIDTH",
        "SHLD_TOTAL_WIDTH", "RECORD_DATE",
    ],
}


def request_json(url: str) -> dict:
    last_error: Exception | None = None
    for attempt in range(4):
        try:
            with urllib.request.urlopen(url, timeout=60) as response:
                value = json.loads(response.read().decode("utf-8-sig"))
            if "error" in value:
                raise RuntimeError(value["error"])
            return value
        except Exception as error:  # network retry boundary
            last_error = error
            if attempt < 3:
                time.sleep(1 + attempt)
    raise RuntimeError(f"request failed after retries: {url}") from last_error


def query_url(service: str, parameters: dict[str, str | int]) -> str:
    return f"{BASE}/{service}/MapServer/0/query?{urllib.parse.urlencode(parameters)}"


def capture_county(service: str, fields: list[str], county: int, routes: set[int]) -> dict:
    route_values = ",".join(f"'{route:04d}'" for route in sorted(routes))
    where = f"CTY_CODE='{county:02d}' AND ST_RT_NO IN ({route_values})"
    count_value = request_json(query_url(service, {
        "where": where, "returnCountOnly": "true", "f": "json",
    }))
    expected = int(count_value["count"])
    features: list[dict] = []
    page_size = 2000
    for offset in range(0, expected, page_size):
        page = request_json(query_url(service, {
            "where": where,
            "outFields": ",".join(fields),
            "returnGeometry": "false",
            "orderByFields": "OBJECTID",
            "resultOffset": offset,
            "resultRecordCount": page_size,
            "f": "json",
        }))
        features.extend(feature["attributes"] for feature in page.get("features", []))
    if len(features) != expected:
        raise ValueError(f"{service} county {county}: expected {expected}, received {len(features)}")
    return {
        "county_code": f"{county:02d}",
        "state_routes": [f"{route:04d}" for route in sorted(routes)],
        "where": where,
        "feature_count": expected,
        "features": features,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--workbook", type=Path, required=True)
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--services", nargs="+", choices=sorted(SERVICES), default=sorted(SERVICES))
    args = parser.parse_args()

    workbook = openpyxl.load_workbook(args.workbook, read_only=True, data_only=True)
    sheet = workbook["tblRumbleStrips"]
    values = sheet.iter_rows(values_only=True)
    headers = list(next(values))
    county_index = headers.index("County #")
    route_index = headers.index("State Route #")
    routes_by_county: dict[int, set[int]] = defaultdict(set)
    for row in values:
        routes_by_county[int(row[county_index])].add(int(row[route_index]))
    if len(routes_by_county) != 67:
        raise ValueError(f"expected 67 workbook counties, found {len(routes_by_county)}")

    args.output_dir.mkdir(parents=True, exist_ok=True)
    for service in args.services:
        fields = SERVICES[service]
        captures = []
        with ThreadPoolExecutor(max_workers=8) as executor:
            futures = {
                executor.submit(capture_county, service, fields, county, routes): county
                for county, routes in routes_by_county.items()
            }
            for future in as_completed(futures):
                captures.append(future.result())
        captures.sort(key=lambda value: value["county_code"])
        feature_count = sum(value["feature_count"] for value in captures)
        features = [feature for capture in captures for feature in capture.pop("features")]
        features.sort(key=lambda value: int(value["OBJECTID"]))
        envelope = {
            "source": f"{BASE}/{service}/MapServer/0",
            "capture_date": "2026-07-28",
            "capture_scope": "county-route pairs present in PennDOT Statewide Rumble Strips 2024 workbook",
            "return_geometry": False,
            "out_fields": fields,
            "county_queries": captures,
            "feature_count": feature_count,
            "features": features,
        }
        output = args.output_dir / f"{service}-workbook-route-universe.json"
        output.write_text(json.dumps(envelope, indent=2) + "\n", encoding="utf-8")
        print(json.dumps({"output": str(output), "features": feature_count}))


if __name__ == "__main__":
    main()
