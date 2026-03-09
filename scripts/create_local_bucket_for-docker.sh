#!/bin/sh

set -eu

BUCKET="${BUCKET:-tasktrackerfiles}"
ENDPOINT="${ENDPOINT:-http://localhost:9000}"
S3_ACCESS_KEY_ID="${S3_ACCESS_KEY_ID:-minioadmin}"
S3_SECRET_ACCESS_KEY="${S3_SECRET_ACCESS_KEY:-minioadmin}"

for _ in $(seq 1 30); do
  STATUS="$(curl -s -o /dev/null -w "%{http_code}" \
    --aws-sigv4 "aws:amz:us-east-1:s3" \
    --user "${S3_ACCESS_KEY_ID}:${S3_SECRET_ACCESS_KEY}" \
    "${ENDPOINT}/${BUCKET}" || true)"

  if [ "${STATUS}" = "200" ]; then
    echo "Bucket exists"
    exit 0
  fi

  if [ "${STATUS}" = "404" ]; then
    echo "Creating bucket..."
    curl -fsS -X PUT \
      --aws-sigv4 "aws:amz:us-east-1:s3" \
      --user "${S3_ACCESS_KEY_ID}:${S3_SECRET_ACCESS_KEY}" \
      "${ENDPOINT}/${BUCKET}"
    exit 0
  fi

  sleep 2
done

echo "MinIO bucket bootstrap failed: ${BUCKET}" >&2
exit 1
