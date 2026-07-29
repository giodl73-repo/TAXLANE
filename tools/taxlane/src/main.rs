use std::process::ExitCode;

mod artifacts;
mod commands;
mod support;
mod types;

pub(crate) use artifacts::*;
pub(crate) use commands::*;
pub(crate) use support::*;
pub(crate) use types::*;

use std::env;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.as_slice() {
        [area, command] if area == "income-tax-outlay" && command == "validate" => {
            run_income_tax_outlay_validation()
        }
        [area, command, flag]
            if area == "income-tax-outlay" && command == "model" && flag == "--check" =>
        {
            run_model_check()
        }
        [area, command] if area == "income-tax-outlay" && command == "model" => run_model_write(),
        [area, command, flag]
            if area == "income-tax-outlay"
                && command == "subfunction-model"
                && flag == "--check" =>
        {
            run_subfunction_model_check()
        }
        [area, command] if area == "income-tax-outlay" && command == "subfunction-model" => {
            run_subfunction_model_write()
        }
        [area, command, flag]
            if area == "income-tax-outlay"
                && command == "subfunction-export"
                && flag == "--check" =>
        {
            run_subfunction_export_check()
        }
        [area, command] if area == "income-tax-outlay" && command == "subfunction-export" => {
            run_subfunction_export_write()
        }
        [area, command, flag]
            if area == "income-tax-outlay" && command == "summary" && flag == "--check" =>
        {
            run_summary_check()
        }
        [area, command] if area == "income-tax-outlay" && command == "summary" => {
            run_summary_write()
        }
        [area, command, flag]
            if area == "income-tax-outlay" && command == "export" && flag == "--check" =>
        {
            run_export_check()
        }
        [area, command] if area == "income-tax-outlay" && command == "export" => run_export_write(),
        [area, command, flag]
            if area == "income-tax-outlay" && command == "manifest" && flag == "--check" =>
        {
            run_manifest_check()
        }
        [area, command] if area == "income-tax-outlay" && command == "manifest" => {
            run_manifest_write()
        }
        [area, command, flag]
            if area == "receipt-source" && command == "table-2-2" && flag == "--check" =>
        {
            run_table_2_2_check()
        }
        [area, command] if area == "receipt-source" && command == "table-2-2" => {
            run_table_2_2_write()
        }
        [area, command, flag]
            if area == "outlay-function" && command == "table-3-1" && flag == "--check" =>
        {
            run_table_3_1_check()
        }
        [area, command] if area == "outlay-function" && command == "table-3-1" => {
            run_table_3_1_write()
        }
        [area, command, flag]
            if area == "outlay-function"
                && command == "table-3-2-national-defense"
                && flag == "--check" =>
        {
            run_table_3_2_national_defense_check()
        }
        [area, command] if area == "outlay-function" && command == "table-3-2-national-defense" => {
            run_table_3_2_national_defense_write()
        }
        [area, command, flag]
            if area == "outlay-composition"
                && command == "table-6-1-national-defense"
                && flag == "--check" =>
        {
            run_table_6_1_national_defense_check()
        }
        [area, command]
            if area == "outlay-composition" && command == "table-6-1-national-defense" =>
        {
            run_table_6_1_national_defense_write()
        }
        [area, command, flag]
            if area == "outlay-function" && command == "table-3-2" && flag == "--check" =>
        {
            run_table_3_2_check()
        }
        [area, command] if area == "outlay-function" && command == "table-3-2" => {
            run_table_3_2_write()
        }
        _ => {
            eprintln!(
                "usage: taxlane-tools income-tax-outlay <validate|model [--check]|subfunction-model [--check]|subfunction-export [--check]|summary [--check]|export [--check]|manifest [--check]>\n       taxlane-tools receipt-source table-2-2 [--check]\n       taxlane-tools outlay-function table-3-1 [--check]\n       taxlane-tools outlay-function table-3-2-national-defense [--check]\n       taxlane-tools outlay-function table-3-2 [--check]\n       taxlane-tools outlay-composition table-6-1-national-defense [--check]"
            );
            ExitCode::from(2)
        }
    }
}

