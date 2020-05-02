#!/bin/bash

print_schema() {
  PORT=10042
  TMP_PG_URL="postgres://postgres:postgres@localhost:${PORT}/postgres"
  SCHEMA_FILE="src/pg/schema.rs"
  CONTAINER_NAME="diesel-print-schema"

  docker stop ${CONTAINER_NAME} || true
  docker run -d --rm --name ${CONTAINER_NAME} -p ${PORT}:5432 postgres
  sleep 5
  diesel migration run --database-url "${TMP_PG_URL}" || true
  diesel print-schema --database-url "${TMP_PG_URL}" > ${SCHEMA_FILE}
  docker stop ${CONTAINER_NAME}

}

help() {
  echo "Usage : ${0} {print, help}"
}

case $1 in
  "print")
    print_schema
    ;;
  "help")
    help
    ;;
  "*" | "")
    help
    ;;
esac
