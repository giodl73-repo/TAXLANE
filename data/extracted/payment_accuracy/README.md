# PaymentAccuracy FY2024 Extraction

Source workbook: `SRC-OMB-PAYMENTACCURACY-FY2024-DATA`.

Run:

```powershell
python data/extracted/payment_accuracy/extract_fy2024.py
```

The extraction preserves 68 program results, the government-wide total, 54
court-confirmed-fraud rows, and 59 agency recovery rows. Program measurement
periods vary. Confirmed fraud and agency recovery tables have different scopes
from estimated improper payments and must not be subtracted from the headline
without a reviewed linkage.
