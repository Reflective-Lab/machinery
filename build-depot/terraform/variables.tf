variable "env" {
  type        = string
  default     = "prod"
  description = "Deployment environment"
  validation {
    condition     = contains(["dev", "staging", "prod"], var.env)
    error_message = "env must be dev, staging, or prod"
  }
}

variable "triggerdev_webhook_url" {
  type        = string
  description = "Trigger.dev endpoint URL — Build-Depot jobs are registered here"
}

variable "sentry_organization" {
  type        = string
  description = "Sentry organization slug"
}

variable "linear_team_id" {
  type        = string
  description = "Linear team ID for the RFL team"
}

variable "watched_repos" {
  type        = list(string)
  default     = ["runtime-runway", "commerce-rails", "converge", "organism", "axiom", "helms", "quorum-sense", "build-depot"]
  description = "GitHub repos to wire with Build-Depot webhooks"
}

variable "watched_sentry_projects" {
  type        = list(string)
  default     = ["runtime-runway", "quorum-sense"]
  description = "Sentry project slugs to wire with Build-Depot webhooks"
}
