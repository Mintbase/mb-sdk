# Mintbase SDK

This repo is supposed to one day replace mintbase core, starting by extracting
events, and later adding common contract functionality to end up in here. Once
the SDK has evolved and the old indexer is replaced with the new and shiny
interop, contracts might be rebased on this SDK instead of `mintbase-deps` and
this repo be made a cargo workspace.

## What this should contain

- Event types
- Common types, e.g. mintbase token, payouts struct
- Smart contract utils

## What this should not contain

- Logging logic
- Indexer-related logic
