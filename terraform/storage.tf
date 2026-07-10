resource "google_storage_bucket" "omnigraph_store" {
  name                        = "reflective-${local.env}-build-depot"
  location                    = "EU"
  storage_class               = "STANDARD"
  uniform_bucket_level_access = true

  versioning {
    enabled = true
  }
}
