terraform {
  required_version = ">= 1.6"

  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.0"
    }
    github = {
      source  = "integrations/github"
      version = "~> 6.0"
    }
    linear = {
      source  = "linear/linear"
      version = "~> 0.1"
    }
    sentry = {
      source  = "jianyuan/sentry"
      version = "~> 0.12"
    }
  }

  backend "gcs" {
    bucket = "reflective-labs-platform-prod-tf-state"
    prefix = "terraform/build-depot/prod"
  }
}

# Read shared GCP coordinates from Runtime-Runway's state.
data "terraform_remote_state" "runway" {
  backend = "gcs"
  config = {
    bucket = "reflective-labs-platform-prod-tf-state"
    prefix = "terraform/platform/prod"
  }
}

locals {
  project = data.terraform_remote_state.runway.outputs.gcp_project_id
  region  = data.terraform_remote_state.runway.outputs.gcp_region
  env     = var.env
}

provider "google" {
  project = local.project
  region  = local.region
}

provider "github" {
  owner = "Reflective-Lab"
}

provider "linear" {}

provider "sentry" {}
