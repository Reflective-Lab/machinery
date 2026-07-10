output "service_account_email" {
  value       = google_service_account.build_depot.email
  description = "Build-Depot SA email — use for GitHub Actions OIDC and Trigger.dev worker config"
}

output "omnigraph_bucket" {
  value       = google_storage_bucket.omnigraph_store.name
  description = "GCS bucket for Omnigraph Lance store — update cluster.yaml storage: gs://<this>"
}

output "secret_ids" {
  value       = { for k, v in google_secret_manager_secret.build_depot : k => v.secret_id }
  description = "Secret Manager secret IDs — populate values after apply"
}
