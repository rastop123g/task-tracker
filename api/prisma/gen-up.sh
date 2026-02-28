npx prisma migrate diff \
  --from-config-datasource \
    --to-schema=prisma/schema.prisma \
    --script

