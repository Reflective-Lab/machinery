# Business & Commerce Architecture

This domain explains how money, entitlement, partner obligations, and commercial
authority move through the system.

## Current Commerce Boundary

Commerce Rails is the commercial authority. It owns the Reflective business
facts around:

- partner accounts and app listings
- customer installations
- subscriptions and entitlement grants
- revenue-share agreements
- transfer intents and payout obligations
- refunds, disputes, webhook receipts, and reconciliation

Stripe is an adapter. Future payment or marketplace providers should remain
behind the Commerce Rails boundary.

## Current Code Surface

| Surface | Home | Role |
|---------|------|------|
| Commerce Rails workspace | `commerce-rails/` | Commercial contracts plus Stripe adapter crates |
| Runtime Runway link | `runtime-runway/` | Runtime operation boundary that may transport commerce events |
| Wolfgang temporary scaffolding | `studio-apps/wolfgang-chat` | Product-local billing code exists as launch scaffolding until shared rails are consumed |
| Helm revenue truths | `bedrock-platform/helms` | Subscription, top-up, ledger, entitlement, and reconciliation truth modeling |

## Money Movement Questions

Use this domain to answer:

- Who pays whom?
- Which system owns commercial truth?
- Which provider state is accepted, normalized, or rejected?
- Which events grant entitlement or create payout obligations?
- Where does reconciliation happen?

## Canonical Links

- [[../business-tech-map|Business / Tech Map]]
- [[../glossary|Glossary]]
- [[../outcome-workbench/kb/Architecture/Truths Layer|Helm Truths Layer]]
- [[../outcome-workbench/kb/Architecture/Converge Application|Helm Converge Application]]
- [[../wolfgang-business/03-business/pricing-and-billing|Wolfgang Pricing and Billing]]
