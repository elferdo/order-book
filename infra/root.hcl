remote_state {
  backend = "local"                     # Minikube is local; adjust if you want S3/DynamoDB later
  config = {
    path = "${get_terragrunt_dir()}/../state/${path_relative_to_include()}.tfstate"
  }
}


