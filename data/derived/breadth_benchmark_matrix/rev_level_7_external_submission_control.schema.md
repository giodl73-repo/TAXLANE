# REV Level 7 external submission control schema

The control record must enumerate every payload file, bundle path, media type,
role, and exact SHA-256 hash. Its bundle ID is the SHA-256 of the ordered
`bundle_path:payload_sha256` lines. The builder must reject missing files,
hash mismatches, path traversal, duplicate bundle paths, or an enabled send.

Requester, authority, signer, approved digest, selected official channels,
transmission, receipt, and response remain null or false until independently
verified. Building a local bundle is not authorization to transmit it.
