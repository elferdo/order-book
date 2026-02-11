# variable "namespace" {
#   description = "K8s namespace for the DB"
#   type        = string
# }
# 
# variable "postgres_user" {
#   description = "Postgres admin username"
#   type        = string
#   default     = "admin"
# }
# 
# variable "postgres_password" {
#   description = "Postgres admin password"
#   type        = string
#   sensitive   = true
# }
# 
# variable "storage_size_gb" {
#   description = "PVC size"
#   type        = number
#   default     = 5
# }
# 
variable "dbname" {
    description = "Name of the postgres service"
    type        = string
}
