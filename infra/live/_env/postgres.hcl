locals {
  # Load the relevant env.hcl file based on where the including unit is.
  # This works because find_in_parent_folders always runs in the context of the unit.
  env_vars = read_terragrunt_config(find_in_parent_folders("env.hcl"))
  env_name = local.env_vars.locals.env

  source_base_dir = "../../../modules/postgres"
}

inputs = {
#  namespace         = "${local.env_name}-db"
  namespace         = "default"
  postgres_user     = "admin"
  postgres_password = get_env("PG_${upper(local.env_name)}_PASSWORD")
  storage_size_gb   = 5
}
