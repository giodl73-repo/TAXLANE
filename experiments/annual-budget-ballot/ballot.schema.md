# Budget Ballot Contract

Each ballot contains one non-negative numeric allocation for every displayed
lane. The computer must reject a ballot unless:

```text
abs(sum(allocation_percent) - 100.0) <= 0.000001
```

The ballot also records the experiment version and information treatment. It
does not record names or infer a voter's fraud beliefs, protected traits, or
real political affiliation.

State aggregation is the arithmetic mean of valid ballots. National Electoral
College aggregation is the Electoral College-weighted mean of state/DC means;
it is not winner-take-all because the object being aggregated is a numeric
budget vector rather than candidate electors.
