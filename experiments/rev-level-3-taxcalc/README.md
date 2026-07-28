# REV Level-3 Tax-Calculator experiment

This experiment uses Tax-Calculator 6.5.1 and its bundled CPS tax-unit data to
estimate the TY2026 federal individual-income-tax liability change from a
uniform ordinary-rate increase. It reports both a static result and a partial-
equilibrium taxable-income response using `behresp`.

Run from the repository root with Python 3.12:

```powershell
py -3.12 -m venv target/rev-l3-venv
target/rev-l3-venv/Scripts/python.exe -m pip install -r experiments/rev-level-3-taxcalc/requirements.txt
target/rev-l3-venv/Scripts/python.exe experiments/rev-level-3-taxcalc/run.py --uplift 7.159 --sub-elasticity 0.25
target/rev-l3-venv/Scripts/python.exe experiments/rev-level-3-taxcalc/run.py --uplift 10.922 --sub-elasticity 0.25 --first-year-ratio 0.7742238946378175
target/rev-l3-venv/Scripts/python.exe experiments/rev-level-3-taxcalc/run.py --uplift 11.0 --sub-elasticity 0.25 --first-year-ratio 0.7742238946378175
```

The output is a model estimate of tax-year liability, not a JCT/CBO score and
not fiscal-year cash receipts. CPS imputations, elasticity choice, timing,
administration, compliance, and macroeconomic effects remain explicit limits.
