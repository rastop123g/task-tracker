BUCKET=tasktrackerfiles
ENDPOINT=http://localhost:9000

# Проверка
STATUS=$(curl -s -o /dev/null -w "%{http_code}" \
  --aws-sigv4 "aws:amz:us-east-1:s3" \
  --user "minioadmin:minioadmin" \
  $ENDPOINT/$BUCKET)

if [ "$STATUS" = "404" ]; then
  echo "Creating bucket..."
  curl -X PUT \
    --aws-sigv4 "aws:amz:us-east-1:s3" \
    --user "minioadmin:minioadmin" \
    $ENDPOINT/$BUCKET
else
  echo "Bucket exists"
fi
