output "postgres_service_name" {
  description = "K8s Service name for the DB"
  value       = helm_release.postgres.name
}

