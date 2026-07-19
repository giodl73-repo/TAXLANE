# Current-law source-custody packet template schema

Schema for `current_law_source_custody_packet_template.v1.draft.json`.

Required top-level fields:

- `record_id`: must equal `current-law-source-custody-packet-template:v1`.
- `record_family`: must equal `current_law_source_custody_packet_template`.
- `version`
- `status`
- `pulse`: must equal `120`.
- Links to the custody batch plan, source-custody preflight, and current-law
  path inventory.
- `template_rules`
- `required_packet_fields`
- `packet_template`
- `readiness_checks`
- `blocked_output_fields`
- `claim_booleans`
- `plain_english_status`

The template may define future packet structure only. All example packet fields
that would contain capture data must remain null, all readiness checks must
remain `ready: false` with `value: null`, and all output fields must remain
null.
