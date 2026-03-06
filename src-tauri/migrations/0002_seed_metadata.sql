INSERT INTO metadata (id, instance_id, schema_version, created_at, updated_at)
VALUES (
    'installation',
    '00000000-0000-7000-8000-000000000001',
    1,
    '1970-01-01T00:00:00Z',
    '1970-01-01T00:00:00Z'
)
ON CONFLICT(id) DO NOTHING;
