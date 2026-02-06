# live/staging/postgres/terragrunt.hcl
include {
  path = find_in_parent_folders("root.hcl")
}

inputs = {
  namespace         = "staging-db"
  postgres_user     = "admin"
  postgres_password = get_env("PG_STAGING_PASSWORD")
  storage_size_gb   = 10
}
