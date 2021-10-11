variable "service_name" {
  type = string
}

variable "stage" {
  type = string
}

variable "region" {
  type = string
}

variable "log_retention_in_days" {
  type    = number
  default = 30
}

variable "log_level" {
  type    = string
  default = "info"
}
