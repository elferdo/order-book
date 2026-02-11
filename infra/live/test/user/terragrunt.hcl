include {
  path = find_in_parent_folders("root.hcl")
}

include "env" {
  path   = "${get_terragrunt_dir()}/../../_env/user.hcl"
  # path   = "../../_env/user.hcl"
  expose = true
}

terraform {
  source = include.env.locals.source_base_dir
}

dependency "postgres" {
  config_path = "../postgres"

  mock_outputs = {
    postgres_service_name = "hola"
  }
}

inputs = {
  dbname = dependency.postgres.outputs.postgres_service_name
}

# inputs = {
#   namespace         = "test-db"
#   postgres_user     = "admin"
#   postgres_password = get_env("PG_TEST_PASSWORD")
#   storage_size_gb   = 5
# }
