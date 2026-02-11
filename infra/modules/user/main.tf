terraform {
  backend "local" {}

  required_version = ">= 1.11.0"
  required_providers {
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "~> 3.0.1"
    }
    helm = {
      source  = "hashicorp/helm"
      version = "~> 3.1.1"
    }
  }
}

# -------------------------------------------------
# Provider – we rely on the kubeconfig that Minikube writes
provider "kubernetes" {
  config_path = "~/.kube/config"
}

provider "helm" {
  kubernetes = {
    config_path = "~/.kube/config"
  }
}
# -------------------------------------------------

resource "kubernetes_deployment_v1" "user" {
  metadata {
    name = "user-service"
    labels = {
      app = "markets"
    }
  }

  spec {
    replicas = 3
    selector {
      match_labels = {
        App = "UserService"
      }
    }
    template {
      metadata {
        labels = {
          App = "UserService"
        }
      }
      spec {
        container {
          image = "user-image:latest"
          name  = "user-service"
          image_pull_policy = "IfNotPresent"

          env {
            name = "DATABASE_URL"
            value = "postgres://admin:ferdo@${var.dbname}-postgresql/postgres"
          }
          port {
            container_port = 80
          }

          resources {
            limits = {
              cpu    = "0.5"
              memory = "512Mi"
            }
            requests = {
              cpu    = "250m"
              memory = "50Mi"
            }
          }
        }
      }
    }
  }
}

