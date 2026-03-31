# AWS

## 1. Bases de Datos
- **RDS (Relational Database Service):** Para bases de datos SQL como PostgreSQL o MySQL. Es un servicio autoadministrado donde no tienes que instalar el software manualmente.
- **DynamoDB:** La opción NoSQL de AWS, ideal para escalabilidad horizontal y estructuras de datos flexibles.
- **DocumentDB:** Funciona como una alternativa compatible con MongoDB dentro del ecosistema de AWS.
- **ElastiCache:** Servicio gestionado para Redis, utilizado principalmente para caché y mejorar el rendimiento de las peticiones.

## 2. Backend (API y Lógica)
- **EC2 (Elastic Compute Cloud):** Servidores virtuales tradicionales (VPS) donde tienes control total para instalar lo que necesites.
- **App Runner:** Una plataforma como servicio (PaaS) más sencilla que automatiza el despliegue y escalado, similar a servicios como Heroku o Railway.
- **ECS y ECR:** Servicios diseñados para trabajar con contenedores de Docker. ECR sirve para guardar las imágenes y ECS para ejecutarlas.
- **AWS Lambda:** Para funciones serverless (sin servidor), ideal para ejecutar trozos de código específicos sin mantener un servidor encendido todo el tiempo.
- **ELB (Elastic Load Balancer)** Te puede ayudar a agrupar lambdas e incluso llamarlas directamente. 

## 3. Frontend
- **Amplify:** Recomendado especialmente para proyectos de Next.js o aplicaciones que requieren integración rápida con otros servicios de AWS.
- **S3 + CloudFront:** La combinación ideal para sitios estáticos (React, Vue, Astro). S3 aloja los archivos y CloudFront los sirve de forma rápida a nivel global.

## 4. Servicios Complementarios
- **Route 53:** Para la gestión de dominios y DNS .
- **IAM:** Crucial para gestionar usuarios, roles y permisos de seguridad dentro de AWS .
- **SES (Simple Email Service):** Para el envío de correos electrónicos desde tu aplicación .
- **Cognito:** Para gestionar la autenticación de usuarios (login con Google, Facebook, etc.).
- **Secrets Manager:** Para guardar de forma segura credenciales y claves de API, evitando dejarlas en texto plano en el código.
