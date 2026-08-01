# Fifteen-Lane Admission Gate v1 Schema

This is the canonical internal disposition frontier for the 15 TAXLANE lanes.
Every lane must have exactly one candidate or downstream function and exactly
one terminal class: `admitted`, `named_trigger`, `active_monitor`, or
`downstream`.

Named triggers and monitors are successful bounded dispositions, not admitted
effects. Downstream functions cannot originate primary savings. A future
evidence event reopens only its named lane; the canonical frontier must then be
versioned and revalidated before any portfolio, NET, REV, or rate consequence.
