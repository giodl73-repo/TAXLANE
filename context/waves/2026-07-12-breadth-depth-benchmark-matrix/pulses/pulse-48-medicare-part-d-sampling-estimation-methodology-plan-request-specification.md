# Pulse 48: Medicare Part D Sampling and Estimation Methodology Plan Request Specification

## Objective

Turn the Pulse 47 access ceiling into a privacy-aware, existing-records-only
request specification without submitting anything or changing external state.

## Source Capture

This pulse captures official CMS filing instructions and the official published
45 CFR Part 5 PDF alongside the existing OMB M-21-19 custody. The HHS request
page was web-verified, but Akamai HTTP 403 blocked local binary custody, so it
receives no source ID or checksum claim.

## Draft Scope

The draft requests the final CY2022/FY2024 Part D S&EMP, OMB checklist,
incorporated appendices and technical attachments, operative version history,
and final approval or transmittal records. It seeks existing records only and
excludes claims, PHI, sampled-PDE identifiers, credentials, raw input data,
production access, and the separate same-cohort recovery track.

Native electronic or searchable formats, rolling release, lawful redaction,
deidentification, and reasonably segregable nonexempt portions are accepted
without predicting release.

## Decision

Record an internal draft with submission status
`draft_not_submitted_owner_authorization_required`. Owner authorization,
requester details, fee decisions, scope review, and one channel remain blocked.
No fee waiver or expedition eligibility is claimed.

## Program Impact

Zero components and zero fields close. Part D remains three closed and five
open, with three closure decisions and five residual gaps. All ten claim and
scoring gates remain false. No request, external message, fee commitment, or
outbound state change occurred.
