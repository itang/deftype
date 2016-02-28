## create db

$ su - postgres

$ psql

```
CREATE DATABASE deftype_dev OWNER dbuser ENCODING 'UTF8';

GRANT ALL PRIVILEGES ON DATABASE deftype_dev to dbuser;
```

## Connect db

$ psql -U dbuser -d deftype_dev -h 127.0.0.1 -p 5432
