resource "google_service_account" "build_depot" {
  account_id   = "build-depot"
  display_name = "Build-Depot"
  description  = "Service account for Build-Depot factory operations (Omnigraph, Trigger.dev)"
}

resource "google_storage_bucket_iam_member" "omnigraph_store" {
  bucket = google_storage_bucket.omnigraph_store.name
  role   = "roles/storage.objectAdmin"
  member = "serviceAccount:${google_service_account.build_depot.email}"
}

resource "google_project_iam_member" "secret_accessor" {
  project = local.project
  role    = "roles/secretmanager.secretAccessor"
  member  = "serviceAccount:${google_service_account.build_depot.email}"
}
