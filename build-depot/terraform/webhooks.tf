# Webhook secret is read back after secrets.tf creates the slot.
# Populate the actual value via: gcloud secrets versions add build-depot-webhook-secret --data-file=-
data "google_secret_manager_secret_version" "webhook_secret" {
  secret = google_secret_manager_secret.build_depot["webhook-secret"].secret_id
}

# GitHub — one webhook per watched repo
resource "github_repository_webhook" "build_depot" {
  for_each   = toset(var.watched_repos)
  repository = each.value

  configuration {
    url          = var.triggerdev_webhook_url
    content_type = "json"
    secret       = data.google_secret_manager_secret_version.webhook_secret.secret_data
    insecure_ssl = false
  }

  events = [
    "pull_request",
    "push",
    "release",
    "check_run",
    "check_suite",
    "deployment",
    "deployment_status",
  ]
  active = true
}

# Linear — one webhook for the RFL team (issue state changes feed the debt-tracker plugin)
resource "linear_webhook" "build_depot" {
  url      = var.triggerdev_webhook_url
  team_id  = var.linear_team_id
  secret   = data.google_secret_manager_secret_version.webhook_secret.secret_data

  resource_types = ["Issue"]
}

# Sentry — one webhook per watched project (incidents feed the incident-gate plugin)
resource "sentry_project_inbound_data_filter" "build_depot" {
  for_each     = toset(var.watched_sentry_projects)
  organization = var.sentry_organization
  project      = each.value
}

resource "sentry_project_plugin" "build_depot_webhook" {
  for_each     = toset(var.watched_sentry_projects)
  organization = var.sentry_organization
  project      = each.value
  plugin       = "webhooks"
  config = {
    urls = var.triggerdev_webhook_url
  }
}
