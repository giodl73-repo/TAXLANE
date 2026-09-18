"""Capture the complete filtered PennDOT RMS universe for the TRN state pilot."""

from __future__ import annotations

import argparse
import json
import time
import urllib.parse
import urllib.request
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path


BASE = "https://gis.penndot.pa.gov/gis/rest/services/opendata"
CAPTURES = {
    "roadwaysegments": {
        "where": "JURIS='1' AND URBAN_RURAL='1' AND DIR_IND='B' AND FAC_TYPE='2' AND LANE_CNT=2",
        "fields": [
            "OBJECTID", "ST_RT_NO", "CTY_CODE", "DISTRICT_NO", "JURIS",
            "SEG_NO", "SEG_LNGTH_FEET", "DIR_IND", "FAC_TYPE", "LANE_CNT",
            "URBAN_RURAL", "YR_BUILT", "YR_RESURF", "TOTAL_WIDTH", "SURF_TYPE",
            "COND_DATE", "ROUGH_INDX", "PVMNT_COND_RATE", "OVERALL_PVMNT_IDX",
            "CUR_AADT", "MAINT_RESPON_IND", "SEG_STATUS", "BIKE_LANE",
            "GOVT_LVL_CTRL", "IRI_YEAR", "OPI_YEAR", "IRI_RATING_TEXT",
            "OPI_RATING_TEXT", "SURFACE_YEAR", "SEGMENT_MILES",
        ],
    },
    "roadwayadmin": {
        "where": "JURIS='1' AND FHWA_FUNC_CLS IN ('3','4','5','6')",
        "fields": [
            "OBJECTID", "ST_RT_NO", "CTY_CODE", "DISTRICT_NO", "JURIS",
            "SEG_BGN", "OFFSET_BGN", "SEG_END", "OFFSET_END", "SEG_LNGTH_FEET",
            "FHWA_FUNC_CLS", "FUNC_CLS", "FED_AID_URBAN_AREA", "SPEED_LIMIT",
            "RECORD_UPDATE",
        ],
    },
    "roadwayshoulder": {
        "where": "JURIS='1'",
        "fields": [
            "OBJECTID", "ST_RT_NO", "CTY_CODE", "DISTRICT_NO", "JURIS",
            "SEG_BGN", "OFFSET_BGN", "SEG_END", "OFFSET_END", "SEG_LNGTH_FEET",
            "COND_DATE", "SHLD_SIDE_IND", "SHLD_TYPE", "SHLD_PAVE_WIDTH",
            "SHLD_TRTMT_DATA", "CURRENT_SHLD_TYPE", "CURRENT_PAVE_WIDTH",
            "SHLD_TOTAL_WIDTH", "RECORD_DATE",
        ],
    },
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
        except Exception as error:
            last_error = error
            if attempt < 3:
                time.sleep(1 + attempt)
    raise RuntimeError(f"request failed after retries: {url}") from last_error


def query_url(service: str, parameters: dict[str, str | int]) -> str:
    return f"{BASE}/{service}/MapServer/0/query?{urllib.parse.urlencode(parameters)}"


def capture_page(service: str, where: str, fields: list[str], offset: int, page_size: int) -> list[dict]:
    value = request_json(query_url(service, {
        "where": where,
        "outFields": ",".join(fields),
        "returnGeometry": "false",
        "orderByFields": "OBJECTID",
        "resultOffset": offset,
        "resultRecordCount": page_size,
        "f": "json",
    }))
    return [feature["attributes"] for feature in value.get("features", [])]


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--services", nargs="+", choices=sorted(CAPTURES), default=sorted(CAPTURES))
    args = parser.parse_args()
    args.output_dir.mkdir(parents=True, exist_ok=True)
    page_size = 2000

    for service in args.services:
        contract = CAPTURES[service]
        where = contract["where"]
        fields = contract["fields"]
        count_response = request_json(query_url(service, {
            "where": where, "returnCountOnly": "true", "f": "json",
        }))
        expected = int(count_response["count"])
        pages: dict[int, list[dict]] = {}
        with ThreadPoolExecutor(max_workers=8) as executor:
            futures = {
                executor.submit(capture_page, service, where, fields, offset, page_size): offset
                for offset in range(0, expected, page_size)
            }
            for future in as_completed(futures):
                pages[futures[future]] = future.result()
        features = [feature for offset in sorted(pages) for feature in pages[offset]]
        features.sort(key=lambda value: int(value["OBJECTID"]))
        if len(features) != expected:
            raise ValueError(f"{service}: expected {expected}, received {len(features)}")
        envelope = {
            "source": f"{BASE}/{service}/MapServer/0",
            "capture_date": "2026-07-28",
            "where": where,
            "return_geometry": False,
            "out_fields": fields,
            "page_size": page_size,
            "feature_count": expected,
            "features": features,
        }
        output = args.output_dir / f"{service}-state-denominator-universe.json"
        output.write_text(json.dumps(envelope, indent=2) + "\n", encoding="utf-8")
        print(json.dumps({"output": str(output), "features": expected}))


if __name__ == "__main__":
    main()
