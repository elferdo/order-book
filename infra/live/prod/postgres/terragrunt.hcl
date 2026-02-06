include {
  path = find_in_parent_folders("root.hcl")
}

include "env" {
  path   = "${get_terragrunt_dir()}/../../_env/postgres.hcl"
  expose = true
}

terraform {
  source = "${include.env.locals.source_base_dir}"
}

