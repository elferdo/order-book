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

# Namespace (creates it if missing)
# resource "kubernetes_namespace_v1" "db_ns" {
#   metadata {
#     name = var.namespace
#   }
# }

# PersistentVolumeClaim for Postgres data
resource "kubernetes_persistent_volume_claim_v1" "pg_data" {
  metadata {
    name      = "postgres-pvc"
    labels = {
      app = "markets"
    }
#    namespace = var.namespace
  }
  spec {
    access_modes = ["ReadWriteOnce"]
    resources {
      requests = {
        storage = "${var.storage_size_gb}Gi"
      }
    }
    storage_class_name = "standard"   # Minikube default SC
  }
}

# Helm chart – Bitnami PostgreSQL (well‑maintained, works on Minikube)
resource "helm_release" "postgres" {
  name       = "postgres-${var.namespace}"
  repository = "https://charts.bitnami.com/bitnami"
  chart      = "postgresql"
#  namespace  = var.namespace
  version    = "18.2.4"   # pin a version you trust

  set = [ {
    name  = "global.postgresql.auth.username"
    value = var.postgres_user
  },

   {
    name  = "global.postgresql.auth.password"
    value = var.postgres_password
  },

   {
    name  = "primary.persistence.existingClaim"
    value = kubernetes_persistent_volume_claim_v1.pg_data.metadata[0].name
  },

  # Disable the chart’s own PVC creation – we supply ours
   {
    name  = "primary.persistence.enabled"
    value = "false"
  },

  # Optional: expose via ClusterIP (default) or LoadBalancer for external access
   {
    name  = "service.type"
    value = "LoadBalancer"
  }
]

  # Wait for pods to become ready before finishing apply
  wait = true
  timeout = 600
}
