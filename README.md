# Mittel Engagement

API para manejar interacciones de usuarios en la plataforma de blogging Mittel.

## Uso

Ejecutar este proyecto solo requiere de Cargo. Se incluye un archivo
`.env.example` con las variables de entorno necesarias.

### Ejemplo

Se puede ejecutar un contenedor con la BD necesaria (MySQL) con el siguiente
comando:

```bash
docker run -d -p 3333:3306 -e 'MYSQL_ROOT_PASSWORD=123' -e 'MYSQL_DATABASE=db' mysql
```

Con esto, se puede usar el siguiente archivo `.env`:

```bash
DATABASE_URL=mysql://root:123@localhost:3333/db
INTERNAL_SECRET_TOKEN=123456
USERS_BASE_URL=<url del microservicio de usuarios>
POSTS_BASE_URL=<url del microservicio de posts>
```
