include {
  path = find_in_parent_folders("root.hcl")
}

terraform {
  source = "../../../modules/user"
}

dependency "db" {
  config_path = "../postgres"
}

# inputs = {
#   namespace         = "test-db"
#   postgres_user     = "admin"
#   postgres_password = get_env("PG_TEST_PASSWORD")
#   storage_size_gb   = 5
# }
