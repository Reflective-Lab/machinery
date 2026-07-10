# Secret slots — values are populated via CI or manually after apply.
# Access pattern: direnv reads from Secret Manager via the build-depot SA.
locals {
  depot_secrets = toset([
    "github-token",
    "linear-token",
    "sentry-token",
    "shipyard-ssh-key",
    "shipyard-token",
    "triggerdev-token",
    "webhook-secret",
  ])
}

resource "google_secret_manager_secret" "build_depot" {
  for_each  = local.depot_secrets
  secret_id = "build-depot-${each.key}"

  replication {
    auto {}
  }
}
