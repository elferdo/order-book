locals {
  # Load the relevant env.hcl file based on where the including unit is.
  # This works because find_in_parent_folders always runs in the context of the unit.
  env_root = find_in_parent_folders("env.hcl")
  env_vars = read_terragrunt_config(local.env_root)
  env_name = local.env_vars.locals.env

  source_base_dir = "../../../modules/user"

}

